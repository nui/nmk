#!/usr/bin/env python
import argparse
import hashlib
import os
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
from datetime import datetime
from os import path
from pathlib import Path


def build_parser():
    parser = argparse.ArgumentParser(prog='build')
    parser.add_argument('-b', '--branch',
                        dest='branch',
                        default='master',
                        help='git branch')
    parser.add_argument('-k', '--keep',
                        dest='keep',
                        action='store_true',
                        default=False,
                        help='do not delete temporary files')
    parser.add_argument('-n', '--no-upload',
                        dest='upload',
                        action='store_false',
                        default=True,
                        help='do not upload to cloud')
    parser.add_argument('-s', '--stable',
                        dest='stable',
                        action='store_true',
                        default=False,
                        help='create bundle for upload to github release')
    # parser.add_argument('tmux_args', nargs=argparse.REMAINDER)
    return parser


def change_directory_to_git_root():
    git_root = Path(__file__).absolute().parent.parent
    os.chdir(git_root)


def clone_repo(branch):
    origin = 'https://github.com/nuimk/nmk.git'
    repo_dir = Path(tempfile.mkdtemp(suffix='.nmk-build'))
    common_kwargs = {
        'stdout': sys.stdout,
        'stderr': sys.stderr,
        'cwd': repo_dir,
    }
    subprocess.run(['git', 'clone', '--quiet', '-b', branch, origin, repo_dir], stdout=sys.stdout, stderr=sys.stderr)
    subprocess.run(['git', 'submodule', 'init'], **common_kwargs)
    subprocess.run(['git', 'submodule', '--quiet', 'update', '--recursive'], **common_kwargs)
    subprocess.run(['git', 'remote', 'set-url', 'origin', origin], **common_kwargs)
    return repo_dir


def clone_vim_plugins(repo):
    pattern = re.compile(r'^(\w\S+)\s+(\S+)')
    plugin_file = open(repo.joinpath('vim', 'plugins'), 'rt')
    plugin_dir = repo.joinpath('vim', 'bundle')
    for line in plugin_file.readlines():
        m = pattern.match(line)
        if m:
            (name, url) = m.groups()
            print(f'Cloning {name}')
            subprocess.run(['git', 'clone', '--quiet', url, name], cwd=plugin_dir)


def generate_buildinfo(repo):
    now = datetime.now()
    out = open(repo.joinpath('.buildinfo'), 'wt')

    kwargs = {
        'stdout': subprocess.PIPE,
        'stderr': sys.stderr,
        'cwd': repo,
    }
    lines = [f"Build on {now}", "Last 10 commits"]
    _ = lambda proc: lines.append(proc.stdout.decode())
    cmd = ["git", "--no-pager", "log", "-n", "10", "--no-color", "--oneline", "--decorate", "--graph"]
    _(subprocess.run(cmd, **kwargs))
    lines.append("Submodules:")
    _(subprocess.run(["git", "submodule", 'status'], **kwargs))
    lines.append("Vim plugins:")
    vim_plugin_dir = repo.joinpath('vim', 'bundle')
    for plugin_name in os.listdir(vim_plugin_dir):
        plugin_repo = vim_plugin_dir.joinpath(plugin_name)
        if path.isdir(plugin_repo):
            proc = subprocess.run(['git', 'rev-parse', '--verify', 'HEAD'],
                                  stdout=subprocess.PIPE, stderr=sys.stderr, cwd=plugin_repo)
            lines.append(f'{proc.stdout.decode().strip()}  {plugin_name}')
    out.write('\n'.join(lines))
    out.close()


delete_cache_files = """#!/bin/sh
#!/bin/sh
set -e
find . -name '*.pyc' -exec rm -f {} +
find . -name __pycache__ -exec rmdir {} +
<.installed-files xargs -0 rm
"""


def generate_more_files(repo):
    with open(repo.joinpath('uninstall.sh'), 'wt') as f:
        f.write(delete_cache_files)
    create_list_files = 'find . ! -type d -print0 | sort --reverse --zero-terminated > .installed-files'
    subprocess.run(create_list_files, shell=True, cwd=repo)


def delete_unwanted_files(repo, ignore_files):
    ignore_files = read_ignore_files(repo)
    (_, tmp_tar_path) = tempfile.mkstemp('.tar')
    os.remove(tmp_tar_path)
    tmp_tar_file = tarfile.open(tmp_tar_path, mode='x')

    def tar_filter(tarinfo):
        ignored = any((
            tarinfo.name.endswith('.git') and tarinfo.isdir(),
            tarinfo.name in ignore_files
        ))
        return tarinfo if not ignored else None

    add_all_to_tar(target_dir=repo, filter=tar_filter, tar_file=tmp_tar_file)

    tmp_tar_file.close()
    new_dir = Path(tempfile.mkdtemp('.nmk-build'))
    subprocess.run(['tar', '-x', '-f', tmp_tar_path], cwd=new_dir)
    os.remove(tmp_tar_path)
    return new_dir


def add_all_to_tar(tar_file, target_dir, filter):
    current_directory = path.abspath(os.curdir)
    os.chdir(target_dir)
    for file in os.listdir(target_dir):
        tar_file.add(file, filter=filter)
    os.chdir(current_directory)


def read_ignore_files(repo):
    ignore_file = repo.joinpath('.archiveignore')
    lines = open(ignore_file, 'rt').readlines()
    files = []
    for line in lines:
        line = line.strip('\n')
        if line.startswith('#') or line == '':
            continue
        files.append(line)
    return files


def create_final_archive(final_dir):
    mtime = datetime.now().timestamp()

    def tar_filter(tarinfo):
        tarinfo.uid = tarinfo.gid = 0
        tarinfo.uname = tarinfo.gname = 'root'
        tarinfo.mtime = mtime
        tarinfo.name = path.join('.nmk', tarinfo.name)
        return tarinfo

    (_, tar_path) = tempfile.mkstemp('.nmk.tar.gz')
    os.remove(tar_path)
    archive = tarfile.open(tar_path, 'x:gz')
    add_all_to_tar(tar_file=archive, target_dir=final_dir, filter=tar_filter)
    archive.close()
    return tar_path


def generate_hash(archive):
    out = open(archive.parent.joinpath(archive.name + '.sha256'), 'wt')
    h = hashlib.sha256()
    h.update(open(archive, 'rb').read())
    out.write(f'{h.hexdigest()} *{archive.name}\n')
    out.close()


def upload(dir):
    prefix = ['gsutil', '-h', "Cache-Control:private, max-age=0, no-transform",
              'cp', '-a', 'public-read']
    cloud_path = 'gs://nmk.nuimk.com/'
    for file in ('nmk.tar.gz', 'nmk.tar.gz.sha256'):
        subprocess.run(prefix + [dir.joinpath(file), cloud_path + file],
                       stdin=sys.stdin, stdout=sys.stdout, stderr=sys.stderr)


def main():
    args = build_parser().parse_args()
    change_directory_to_git_root()
    repo = clone_repo(args.branch)
    clone_vim_plugins(repo)

    generate_buildinfo(repo)
    new_path = delete_unwanted_files(repo, ignore_files=read_ignore_files(repo))
    generate_more_files(new_path)
    tmp_archive = create_final_archive(new_path)
    shutil.rmtree(repo)
    shutil.rmtree(new_path)
    tmp_dir = Path(tempfile.mkdtemp('.nmk-build'))
    archive = tmp_dir.joinpath('nmk.tar.gz')
    shutil.move(tmp_archive, archive)
    generate_hash(archive)
    if args.upload:
        upload(tmp_dir)
    print(tmp_dir)


if __name__ == '__main__':
    main()
