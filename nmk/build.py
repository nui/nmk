#!/usr/bin/env python3
# This script support python 3.6 and later

import argparse
import logging
import lzma
import os
import re
import subprocess
from pathlib import Path
from shutil import copyfile

AMD64_LINUX_MUSL = 'x86_64-unknown-linux-musl'
ARM64_LINUX_MUSL = 'aarch64-unknown-linux-musl'
ARM_LINUX_MUSL = 'arm-unknown-linux-musleabi'

ARMV7_LINUX_MUSL = 'armv7-unknown-linux-musleabihf'

SCRIPT_DIR = Path(os.path.realpath(__file__)).parent
GIT_ROOT_DIR = SCRIPT_DIR
DIST_DIR = GIT_ROOT_DIR / 'dist'
TARGET_DIR = GIT_ROOT_DIR / 'target'

TARGET_TRIPLE = {
    'amd64': AMD64_LINUX_MUSL,
    'arm64': ARM64_LINUX_MUSL,
    # We don't use hard-float so it fine to build without it
    'arm': ARM_LINUX_MUSL,
    'armv7': ARMV7_LINUX_MUSL,
}


def build_parser():
    parser = argparse.ArgumentParser(prog='build.py')
    parser.add_argument('-v', '--verbose',
                        dest='verbosity',
                        action='count',
                        default=0,
                        help='Request verbose logging')
    parser.add_argument('--lto',
                        dest='lto',
                        action='store_true',
                        default=False,
                        help="Sets link-time optimization to true")
    parser.add_argument('--strip',
                        dest='strip',
                        action='store_true',
                        default=False,
                        help="Strip build")
    parser.add_argument('--dist',
                        dest='dist',
                        action='store_true',
                        default=False,
                        help="Copy to dist/")
    parser.add_argument('--target',
                        dest='target',
                        default='amd64',
                        choices=TARGET_TRIPLE.keys(),
                        help="Set build target")
    parser.add_argument('args', nargs=argparse.REMAINDER)
    return parser


def invalidate_cache():
    """
    Touch some files to force invalidate cache.

    We need to do this to get correct build time & commit id embedded in binary.
    """
    target_source_files = [
        'src/bin/nmk/main.rs',
        'src/bin/nmkup/main.rs',
        # lib doesn't need to invalidate because it doesn't use any env!/option_env!
        # 'src/nmk/lib.rs',
    ]
    logging.debug("Invalidating cache")
    for file in target_source_files:
        os.utime(file)


def build_rust_flags(target, strip):
    flags = []
    if strip:
        flags += ['-C', 'link-arg=-s']
    if target == ARM64_LINUX_MUSL:
        # See https://github.com/rust-lang/rust/issues/46651#issuecomment-433611633
        flags += ['-C', 'link-arg=-lgcc', '-C', 'target-feature=+crt-static']
    return " ".join(flags)


def build_release(target, strip=False, lto=False, commit_id=None):
    rust_flags = build_rust_flags(target, strip=strip)
    env = {
        'RUSTFLAGS': rust_flags
    }
    if lto:
        env['CARGO_PROFILE_RELEASE_LTO'] = 'true'
    if commit_id:
        env['GIT_SHORT_SHA'] = commit_id
    args = ['cross', 'build', '--release', '--target', target]
    logging.info("Building %s target", target)
    logging.debug("env: %s", env)
    logging.debug("cmd: %s", " ".join(args))
    subprocess.call(args, env=dict(os.environ.copy(), **env))


def get_version_from_manifest(manifest_path):
    prog = re.compile(r'^version\s*=\s*"(.*)"')
    with open(manifest_path, 'rt') as f:
        for line in f:
            m = prog.match(line)
            if m is not None:
                return m.groups()[0]
    return None


def get_release_dir(target_triple):
    return GIT_ROOT_DIR / 'target' / target_triple / 'release'


def get_build_commit_id():
    args = ['git', 'rev-parse', '--short', 'HEAD']
    try:
        return subprocess.check_output(args).decode().strip()
    except subprocess.SubprocessError:
        return None


def dist(target, release_dir):
    for binary in ('nmk', 'nmkup'):
        data = open(release_dir / binary, mode='rb').read()
        with lzma.open(DIST_DIR / f'{binary}-{target}.xz', mode='wb') as f:
            f.write(data)
    copyfile(release_dir / 'nmkup', str(DIST_DIR / f'nmkup-{target}'))


def setup_logging(verbosity):
    level = logging.DEBUG if verbosity > 0 else logging.INFO
    logging.basicConfig(format='%(message)s', level=level)


def main():
    opt = build_parser().parse_args()
    setup_logging(opt.verbosity)
    target = TARGET_TRIPLE.get(opt.target)
    commit_id = get_build_commit_id()
    invalidate_cache()
    build_release(target, lto=opt.lto, commit_id=commit_id, strip=opt.strip)
    release_dir = get_release_dir(target)
    DIST_DIR.mkdir(exist_ok=True)
    if opt.dist:
        dist(target=target, release_dir=release_dir)


if __name__ == '__main__':
    main()
