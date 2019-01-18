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
    return parser


def clone_repo(branch):
    origin = 'https://github.com/nuimk/nmk.git'
    repo_dir = Path(tempfile.mkdtemp(suffix=TMPDIR_SUFFIX))
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


# Generate files that need information from git
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
        if plugin_repo.is_dir():
            proc = subprocess.run(['git', 'rev-parse', '--verify', 'HEAD'],
                                  stdout=subprocess.PIPE, stderr=sys.stderr, cwd=plugin_repo)
            lines.append(f'{proc.stdout.decode().strip()}  {plugin_name}')
    out.write('\n'.join(lines) + '\n')
    out.close()


delete_cache_files = r"""#!/bin/sh
#!/bin/sh
set -e
find . -name '*.pyc' -exec rm -f {} +
find . -name __pycache__ -exec rmdir {} +
<.installed-files xargs -0 rm
"""


def generate_more_files(workdir):
    # add uninstaller script
    with open(workdir.joinpath('uninstall.sh'), 'wt') as f:
        f.write(delete_cache_files)
    create_list_files = 'find . ! -type d -print0 | sort --reverse --zero-terminated > .installed-files'
    # create a list of bundled files
    subprocess.run(create_list_files, shell=True, cwd=workdir)
    # unset write permission to get warning message on update read-only files
    subprocess.run('find . -type f -exec chmod ugo-w {} +', shell=True, cwd=workdir)


def delete_unwanted_files(repo, ignore_files):
    ignore_files = list_ignored_files(repo)
    (_, tmp_tar_path) = tempfile.mkstemp('.tar')
    os.remove(tmp_tar_path)
    tmp_tar_file = tarfile.open(tmp_tar_path, mode='x')

    def tar_filter(tarinfo):
        ignored = any((
            tarinfo.name.endswith('.git') and tarinfo.isdir(),
            tarinfo.name in ignore_files
        ))
        return tarinfo if not ignored else None

    add_all_to_tar(src=repo, filter=tar_filter, archive=tmp_tar_file)

    tmp_tar_file.close()
    tmp_dir = Path(tempfile.mkdtemp(TMPDIR_SUFFIX))
    subprocess.run(['tar', '-x', '-f', tmp_tar_path], cwd=tmp_dir)
    os.remove(tmp_tar_path)
    return tmp_dir


def add_all_to_tar(archive, src, filter):
    current_directory = Path(os.curdir).absolute()
    os.chdir(src)
    for file in os.listdir(src):
        archive.add(file, filter=filter)
    os.chdir(current_directory)


def list_ignored_files(repo):
    ignore_file = repo.joinpath('.archiveignore')
    lines = open(ignore_file, 'rt').readlines()
    files = []
    for line in lines:
        line = line.strip('\n')
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

    (_, tar_path) = tempfile.mkstemp('.nmk.tar.gz')
    os.remove(tar_path)
    archive = tarfile.open(tar_path, 'x:gz')
    add_all_to_tar(archive=archive, src=workdir, filter=tar_filter)
    archive.close()
    return tar_path


def generate_hash(archive):
    out = open(archive.parent.joinpath(archive.name + '.sha256'), 'wt')
    h = hashlib.sha256()
    h.update(open(archive, 'rb').read())
    out.write(f'{h.hexdigest()} *{archive.name}\n')
    out.close()


def upload(workdir):
    prefix = ['gsutil', '-h', "Cache-Control:private, max-age=0, no-transform",
              'cp', '-a', 'public-read']
    cloud_path = 'gs://nmk.nuimk.com/'
    for file in ('nmk.tar.gz', 'nmk.tar.gz.sha256'):
        subprocess.run(prefix + [workdir.joinpath(file), cloud_path + file],
                       stdin=sys.stdin, stdout=sys.stdout, stderr=sys.stderr).check_returncode()


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

    tmp_dir = delete_unwanted_files(repo, ignore_files=list_ignored_files(repo))
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
    if args.stable:
        sign_archive_and_open_explorer(tmp_dir)
    print(tmp_dir)


if __name__ == '__main__':
    main()
