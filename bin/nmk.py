"""
Run on Python 2.6.6 and later
"""

import json
import logging
import os
import subprocess
import sys
import tempfile
import time
from os import environ
from os import path


import argparse
import six

if sys.version_info[0:2] >= (3, 3):
    from shutil import which as find_executable
else:
    from distutils.spawn import find_executable

PY26 = sys.version_info[0:2] == (2, 6)
UNICODE_NAME = 'en_US.UTF-8' if sys.platform == 'darwin' else 'C.UTF-8'


def build_parser():
    parser = argparse.ArgumentParser(prog='nmk')
    parser.add_argument('-2',
                        dest='force256color',
                        action='store_true',
                        default=False,
                        help='force 256 colors terminal')
    parser.add_argument('-8',
                        dest='force8color',
                        action='store_true',
                        default=False,
                        help='force 8 colors terminal')
    parser.add_argument('-L', '--socket',
                        dest='socket',
                        default='nmk',
                        help='use a different tmux socket name')
    parser.add_argument('-l', '--login',
                        dest='login',
                        action='store_true',
                        default=False,
                        help='start a login shell')
    parser.add_argument('-u', '--unicode',
                        dest='unicode',
                        action='store_true',
                        default=False,
                        help='export LANG={0}'.format(UNICODE_NAME))
    parser.add_argument('--force-unicode',
                        dest='force_unicode',
                        action='store_true',
                        default=False,
                        help='export LC_ALL={0}'.format(UNICODE_NAME))
    parser.add_argument('--detach-on-destroy',
                        dest='detach_on_destroy',
                        action='store_const',
                        const='on',
                        default='off',
                        help='detach the client when the session is destroyed')
    parser.add_argument('--no-autofix',
                        dest='autofix',
                        action='store_false',
                        default=True,
                        help='disable automatically fix')
    parser.add_argument('--inception',
                        dest='inception',
                        action='store_true',
                        default=False,
                        help='allow tmux nested sessions')
    parser.add_argument('-d', '--debug',
                        dest='debug',
                        action='store_true',
                        default=False,
                        help='print debug log')
    parser.add_argument('tmux_args', nargs=argparse.REMAINDER)
    return parser


if PY26:
    def check_output(args):
        """
        Replacement of subprocess.check_output in python2.6
        """
        stdout = tempfile.TemporaryFile()
        subprocess.call(args, stdout=stdout)
        stdout.seek(0)
        return stdout.read()
else:
    check_output = subprocess.check_output


def run_get_process_id():
    output = check_output(('sh', '-c', 'echo $$'))
    if isinstance(output, six.binary_type):
        output = output.decode()
    return int(output)


def setup_logging(debug):
    level = logging.DEBUG if debug else logging.WARNING
    logging.basicConfig(format='%(levelname)s:%(message)s',
                        level=level)


def python_info():
    logging.debug('python:sys.executable:' + sys.executable)
    logging.debug('python:sys.version\n' + sys.version)


def setup_path(nmk_dir):
    """
    Setup PATH environment.
      - prepend NMK_DIR/bin and NMK_DIR/local/bin
    """
    nmk_paths = [
        path.join(nmk_dir, 'bin'),
        path.join(nmk_dir, 'local', 'bin')
    ]
    paths = nmk_paths + [p for p in environ['PATH'].split(os.pathsep) if p not in nmk_paths]

    for i, p in enumerate(paths, start=1):
        logging.debug('path[{0:02d}]:{1}'.format(i, p))
    environ['PATH'] = os.pathsep.join(paths)
    logging.debug('PATH:' + environ['PATH'])


def check_dependencies():
    for binary in ('tmux', 'zsh'):
        if not find_executable(binary):
            logging.error('{0} not found'.format(binary))
            sys.exit(1)


def parse_cgroup(cgroup_file):
    with open(cgroup_file) as f:
        for line in f:
            hierarchy_id, subsystems, control_group = line.strip().split(':')
            yield control_group


def is_inside_container():
    cgroup_file = '/proc/1/cgroup'
    if path.exists(cgroup_file):
        control_groups = parse_cgroup(cgroup_file)
        in_docker = any((g.startswith('/docker') for g in control_groups))
    else:
        in_docker = False
    return in_docker


def setup_terminal(args):
    is_container = is_inside_container()
    support_256color = any((
        args.force256color,
        environ.get('TERM') in ('cygwin', 'gnome-256color', 'putty', 'screen-256color', 'xterm-256color'),
        environ.get('COLORTERM') in ('gnome-terminal', 'rxvt-xpm', 'xfce4-terminal'),
        args.autofix and is_container,
    ))

    if is_container:
        logging.debug('detect docker container')

    use_256color = not args.force8color and support_256color
    environ['NMK_TMUX_DEFAULT_TERMINAL'] = 'screen-256color' if use_256color else 'screen'
    environ['NMK_TMUX_256_COLOR'] = "1" if use_256color else "0"


def setup_environment(args, nmk_dir, tmux_version):
    initvim = path.join(nmk_dir, 'vim/init.vim')
    zdotdir = path.join(nmk_dir, 'zsh')

    environ['NMK_DIR'] = nmk_dir
    environ['NMK_TMUX_DEFAULT_SHELL'] = find_executable('zsh')
    environ['NMK_TMUX_DETACH_ON_DESTROY'] = args.detach_on_destroy
    environ['NMK_TMUX_HISTORY'] = path.join(nmk_dir, 'tmux', '.tmux_history')
    environ['NMK_TMUX_VERSION'] = tmux_version
    environ['VIMINIT'] = 'source {0}'.format(initvim.replace(' ', r'\ '))
    environ['ZDOTDIR'] = zdotdir

    if 'VIRTUAL_ENV' in environ:
        del environ['VIRTUAL_ENV']
        logging.debug('unset VIRTUAL_ENV')

    if args.unicode or (args.autofix and 'LANG' not in environ):
        environ['LANG'] = UNICODE_NAME
        logging.debug('set LANG = ' + UNICODE_NAME)

    if args.force_unicode:
        environ['LC_ALL'] = UNICODE_NAME
        logging.debug('set LC_ALL = ' + UNICODE_NAME)


def setup_zsh(args, nmk_dir):
    has_local_zsh = path.exists(path.join(nmk_dir, 'local', 'bin', 'zsh'))

    # Some linux distribution global zprofile contains a line that will source everything
    # from /etc/profile. And they do reset $PATH completely.
    # It makes PATH set by nmk unusable
    bad_global_rcs = any((
        path.exists('/etc/alpine-release'),
        path.exists('/etc/arch-release'),
    ))

    no_global_rcs = all((
        args.autofix,
        bad_global_rcs,
        not has_local_zsh,
    ))

    if no_global_rcs:
        logging.debug('ignore zsh global configuration')

    environ['NMK_ZSH_GLOBAL_RCS'] = "0" if no_global_rcs else "1"


def setup_prefer_editor():
    prefer_editors = ('nvim', 'vim')
    EDITOR = 'EDITOR'
    if EDITOR not in environ:
        for prog in prefer_editors:
            if find_executable(prog):
                environ[EDITOR] = prog
                logging.debug('set EDITOR = ' + prog)
                break


def add_local_library(nmk_dir):
    LD_LIBRARY_PATH = 'LD_LIBRARY_PATH'

    local_lib_dir = path.join(nmk_dir, 'local', 'lib')
    if path.isdir(local_lib_dir):
        library_path = environ.get(LD_LIBRARY_PATH)
        library_paths = library_path.split(os.pathsep) if library_path else []
        if local_lib_dir not in library_paths:
            library_paths.insert(0, local_lib_dir)
            environ[LD_LIBRARY_PATH] = os.pathsep.join(library_paths)
            logging.debug('prepend ' + local_lib_dir + ' to ' + LD_LIBRARY_PATH)
            logging.debug(LD_LIBRARY_PATH + ' = ' + environ[LD_LIBRARY_PATH])


def get_tmux_version():
    output = check_output(('tmux', '-V'))
    if isinstance(output, six.binary_type):
        output = output.decode()
    return output.split()[1]


def get_tmux_conf(version, tmux_dir):
    logging.debug('using tmux {0}'.format(version))
    conf = path.join(tmux_dir, '{0}.conf'.format(version))
    if not path.exists(conf):
        logging.error('tmux {0} is unsupported'.format(version))
        sys.exit(1)
    return conf


def is_server_running(socket):
    devnull = open(os.devnull, 'w')
    running = 0 == subprocess.call(('tmux', '-L', socket, 'list-sessions'),
                                   stdout=devnull,
                                   stderr=devnull)
    status = "found" if running else "not found"
    logging.debug("{0} server running on socket '{1}'".format(status, socket))
    return running


def execvp(file, args):
    logging.debug('os.execvp args: ' + str(args))
    sys.stdout.flush()
    sys.stderr.flush()
    os.execvp(file, args)


def exec_tmux(args, tmux_conf, start_time):
    exec_args = ('tmux',)
    socket = args.socket
    exec_args += ('-L', socket)
    if args.force256color:
        exec_args += ('-2',)
    tmux_args = args.tmux_args[:]
    # If -- is used to separated between tmux and nmk parameters, don't send it to tmux
    if tmux_args and tmux_args[0] == '--':
        tmux_args.pop(0)
    if is_server_running(socket=socket):
        if tmux_args:
            exec_args += tuple(tmux_args)
        else:
            if 'TMUX' in environ and not args.inception:
                logging.error('add --inception to allow nested tmux sessions')
                sys.exit(1)
            exec_args += ('attach',)
    else:
        # start tmux server
        exec_args += ('-f', tmux_conf) + tuple(tmux_args)
    print_time_usage(start_time)
    execvp('tmux', exec_args)


def start_login_shell(args, tmux_conf, start_time):
    exec_args = ('tmux',)
    if args.force256color:
        exec_args += ('-2',)
    exec_args += ('-f', tmux_conf, '-c', 'exec zsh --login')
    print_time_usage(start_time)
    execvp('tmux', exec_args)


def print_time_usage(start_time):
    logging.debug("nmk.py pre exec time = {0} seconds".format(time.time() - start_time))


def clear_temp_env(nmk_dir):
    with open(path.join(nmk_dir, 'config.json'), 'rt') as f:
        envs = json.loads(f.read())['tmuxSettingEnvs']
        for env in envs:
            del environ[env]


def main():
    start_time = time.time()
    args = build_parser().parse_args()
    if args.debug:
        start_pid = run_get_process_id()
    setup_logging(debug=args.debug)
    python_info()
    nmk_dir = path.dirname(path.dirname(path.abspath(__file__)))
    tmux_dir = path.join(nmk_dir, 'tmux')
    add_local_library(nmk_dir=nmk_dir)
    setup_path(nmk_dir=nmk_dir)
    check_dependencies()
    tmux_version = get_tmux_version()
    setup_terminal(args=args)
    setup_environment(args=args, nmk_dir=nmk_dir, tmux_version=tmux_version)
    setup_zsh(args=args, nmk_dir=nmk_dir)
    setup_prefer_editor()
    tmux_conf = path.relpath(get_tmux_conf(tmux_version, tmux_dir))
    if args.debug:
        end_pid = run_get_process_id()
        logging.debug('created {0} processes during initialization'.format(end_pid - start_pid - 1))
    if args.login:
        clear_temp_env(nmk_dir=nmk_dir)
        start_login_shell(args=args, tmux_conf=tmux_conf, start_time=start_time)
    else:
        exec_tmux(args=args, tmux_conf=tmux_conf, start_time=start_time)


if __name__ == '__main__':
    main()
