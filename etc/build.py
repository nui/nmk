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
from pathlib import Path

TMPDIR_SUFFIX = '.nmk-build'


def build_parser():
    parser = argparse.ArgumentParser(prog='build')
    parser.add_argument('-b', '--branch',
                        dest='branch',
                        default='master',
                        help='git branch')
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
    parser.add_argument('--copy-to',
                        dest='copy_to',
                        help='copy build result to this directory')
    return parser


def clone_repo(branch):
    repo = Path(tempfile.mkdtemp(suffix=TMPDIR_SUFFIX))
    subprocess.run(['git', 'clone', '--recursive', '-b', branch,
                    'https://github.com/nuimk/nmk.git', repo])
    return repo


def read_plugin_tuple(repo):
    pattern = re.compile(r'^(\w\S+)\s+(\S+)')
    plugin_file = open(repo.joinpath('vim', 'plugins'), 'rt')
    for line in plugin_file.readlines():
        m = pattern.match(line)
        if m:
            yield m.groups()


def clone_vim_plugins(repo):
    bundle_dir = repo.joinpath('vim', 'bundle')
    for (name, url) in read_plugin_tuple(repo):
        print(f'Cloning {name}')
        subprocess.run(['git', 'clone', '--quiet', url, name], cwd=bundle_dir)


# Generate files that need information from git
def generate_buildinfo(repo):
    kwargs = {
        'stdout': subprocess.PIPE,
        'cwd': repo,
    }

    lines = [f"Build on {datetime.now()}", "Last 10 commits"]
    _ = lambda p: lines.append(p.stdout.decode())

    cmd = ["git", "--no-pager", "log", "-n", "10", "--no-color", "--oneline", "--decorate", "--graph"]
    _(subprocess.run(cmd, **kwargs))

    lines.append("Git submodules:")
    _(subprocess.run(["git", "submodule", 'status'], **kwargs))

    lines.append("Vim plugins:")
    vim_bundle_dir = repo.joinpath('vim', 'bundle')
    for (plugin_name, _) in read_plugin_tuple(repo):
        plugin_repo = vim_bundle_dir.joinpath(plugin_name)
        proc = subprocess.run(['git', 'rev-parse', '--verify', 'HEAD'],
                              stdout=subprocess.PIPE, cwd=plugin_repo)
        head = proc.stdout.decode().strip()
        lines.append(f'{head}  {plugin_name}')
    with open(repo.joinpath('.buildinfo'), 'wt') as out:
        out.write('\n'.join(lines) + '\n')


delete_cache_files = r"""#!/bin/sh
set -e
find . -name '*.pyc' -exec rm -f {} +
find . -name __pycache__ -exec rmdir {} +
<.installed-files xargs -0 rm
"""


def generate_more_files(workdir):
    # add uninstaller script
    with open(workdir.joinpath('uninstall.sh'), 'wt') as f:
        f.write(delete_cache_files)
    # create a list of bundled files
    create_list_files = 'find . ! -type d -print0 | sort --reverse --zero-terminated > .installed-files'
    subprocess.run(create_list_files, shell=True, cwd=workdir)
    # unset write permission to get warning message on update read-only files
    subprocess.run('find . -type f -exec chmod ugo-w {} +', shell=True, cwd=workdir)


def delete_unwanted_files(repo):
    (_, archive_path) = tempfile.mkstemp('.tar')
    os.remove(archive_path)

    with tarfile.open(archive_path, mode='x') as archive:
        def tar_filter(tarinfo):
            ignored = any((
                tarinfo.name.endswith('.git') and tarinfo.isdir(),
                tarinfo.name in list_ignored_files(repo)
            ))
            return tarinfo if not ignored else None
        add_all_to_tar(src=repo, filter=tar_filter, archive=archive)
    tmp_dir = Path(tempfile.mkdtemp(TMPDIR_SUFFIX))
    subprocess.run(['tar', '-x', '-f', archive_path], cwd=tmp_dir)
    os.remove(archive_path)
    return tmp_dir


def add_all_to_tar(archive, src, filter):
    cwd = Path(os.curdir).absolute()
    os.chdir(src)
    for file in os.listdir(src):
        archive.add(file, filter=filter)
    os.chdir(cwd)


def list_ignored_files(repo):
    ignore_file = repo.joinpath('.archiveignore')
    lines = [line.strip() for line in open(ignore_file, 'rt').readlines()]
    files = []
    for line in lines:
        if line.startswith('#') or line == '':
            continue
        files.append(line)
    return files


def create_final_archive(workdir):
    mtime = datetime.now().timestamp()

    def tar_filter(tarinfo):
        tarinfo.uid = tarinfo.gid = 0
        tarinfo.uname = tarinfo.gname = 'root'
        tarinfo.mtime = mtime
        tarinfo.name = str(Path('.nmk').joinpath(tarinfo.name))
        return tarinfo

    (_, archive_path) = tempfile.mkstemp('.nmk.tar.gz')
    os.remove(archive_path)
    with tarfile.open(archive_path, 'x:gz') as archive:
        add_all_to_tar(archive=archive, src=workdir, filter=tar_filter)
    return archive_path


def generate_hash(file):
    h = hashlib.sha256()
    h.update(open(file, 'rb').read())
    with open(file.parent.joinpath(file.name + f'.{h.name}'), 'wt') as out:
        out.write(f'{h.hexdigest()} *{file.name}\n')


def upload(workdir):
    prefix = ('gsutil',
              '-h', 'Cache-Control:private, max-age=0, no-transform',
              'cp')
    for file in ('nmk.tar.gz', 'nmk.tar.gz.sha256'):
        subprocess.run(prefix + (workdir.joinpath(file), f'gs://nmk.nuimk.com/{file}')).check_returncode()


def copy_to(workdir, target):
    for file in ('nmk.tar.gz', 'nmk.tar.gz.sha256'):
        shutil.copy(workdir.joinpath(file), target)


def sign_archive_and_open_explorer(workdir):
    for i in range(0, 3):
        try:
            subprocess.run(['gpg', '-b', '-u', '0x28B07F9036262EEF4D5B2B21B837E20D47A47347',
                            workdir.joinpath('nmk.tar.gz')]).check_returncode()
        except subprocess.CalledProcessError:
            continue
        break
    if sys.platform == 'darwin':
        subprocess.run(['open', workdir])


def main():
    args = build_parser().parse_args()

    repo = clone_repo(args.branch)
    clone_vim_plugins(repo)
    generate_buildinfo(repo)

    tmp_dir = delete_unwanted_files(repo)
    shutil.rmtree(repo)
    generate_more_files(tmp_dir)
    tmp_archive = create_final_archive(tmp_dir)
    shutil.rmtree(tmp_dir)

    tmp_dir = Path(tempfile.mkdtemp(TMPDIR_SUFFIX))
    archive = tmp_dir.joinpath('nmk.tar.gz')
    shutil.move(tmp_archive, archive)
    generate_hash(archive)
    if args.upload:
        upload(tmp_dir)
    if args.copy_to:
        copy_to(workdir=tmp_dir, target=args.copy_to)
    if args.stable:
        sign_archive_and_open_explorer(tmp_dir)
    print(tmp_dir)


if __name__ == '__main__':
    main()
