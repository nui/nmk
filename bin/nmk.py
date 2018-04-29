"""
Run on Python 2.6.6 and later
"""

import logging
import os
import subprocess
import sys
import tempfile
from os import environ as env
from os import path

import argparse
import six

if sys.version_info[0:2] >= (3, 3):
    from shutil import which as find_executable
else:
    from distutils.spawn import find_executable

PY26 = sys.version_info[0:2] == (2, 6)
UNICODE_NAME = 'C.UTF-8'


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
                        help='set tmux socket name')
    parser.add_argument('-l', '--login',
                        dest='login',
                        action='store_true',
                        default=False,
                        help='Start login shell instead of tmux')
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


def get_process_id():
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
    paths = [
                path.join(nmk_dir, 'bin'),
                path.join(nmk_dir, 'local', 'bin')
            ] + env['PATH'].split(os.pathsep)
    for i, p in enumerate(paths, start=1):
        logging.debug('path[{0:02d}]:{1}'.format(i, p))
    env['PATH'] = os.pathsep.join(paths)
    logging.debug('PATH:' + env['PATH'])


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


def is_inside_docker():
    cgroup_file = '/proc/1/cgroup'
    if path.exists(cgroup_file):
        control_groups = parse_cgroup(cgroup_file)
        in_docker = all((g != '/' for g in control_groups))
    else:
        in_docker = False
        logging.error("Couldn't read {0}".format(cgroup_file))
    return in_docker


def setup_terminal(args):
    support_256color = any((
        args.force256color,
        env.get('TERM') in ('cygwin', 'gnome-256color', 'putty', 'screen-256color', 'xterm-256color'),
        env.get('COLORTERM') in ('gnome-terminal', 'rxvt-xpm', 'xfce4-terminal'),
        args.autofix and is_inside_docker(),
    ))

    use_256color = not args.force8color and support_256color
    env['NMK_TMUX_DEFAULT_TERMINAL'] = 'screen-256color' if use_256color else 'screen'
    env['NMK_TMUX_256_COLOR'] = "1" if use_256color else "0"


def setup_environment(args, nmk_dir, tmux_version):
    initvim = path.join(nmk_dir, 'vim/init.vim')
    zdotdir = path.join(nmk_dir, 'zsh')

    env['NMK_DIR'] = nmk_dir
    env['NMK_TMUX_DEFAULT_SHELL'] = find_executable('zsh')
    env['NMK_TMUX_DETACH_ON_DESTROY'] = args.detach_on_destroy
    env['NMK_TMUX_HISTORY'] = path.join(nmk_dir, 'tmux', '.tmux_history')
    env['NMK_TMUX_VERSION'] = tmux_version
    env['VIMINIT'] = 'source {0}'.format(initvim.replace(' ', r'\ '))
    env['ZDOTDIR'] = zdotdir

    if 'VIRTUAL_ENV' in env:
        del env['VIRTUAL_ENV']
        logging.debug('unset VIRTUAL_ENV')

    if args.unicode or (args.autofix and 'LANG' not in env):
        env['LANG'] = UNICODE_NAME
        logging.debug('set LANG = ' + UNICODE_NAME)

    if args.force_unicode:
        env['LC_ALL'] = UNICODE_NAME
        logging.debug('set LC_ALL = ' + UNICODE_NAME)


def setup_prefer_editor():
    prefer_editors = ('nvim', 'vim')
    if 'EDITOR' not in env:
        for prog in prefer_editors:
            if find_executable(prog):
                env['EDITOR'] = prog
                logging.debug('set EDITOR = ' + prog)
                break


def add_local_library(nmk_dir):
    LD_LIBRARY_PATH = 'LD_LIBRARY_PATH'

    local_lib_dir = path.join(nmk_dir, 'local', 'lib')
    if path.isdir(local_lib_dir):
        library_path = env.get(LD_LIBRARY_PATH)
        library_paths = library_path.split(os.pathsep) if library_path else []
        if local_lib_dir not in library_paths:
            library_paths.insert(0, local_lib_dir)
            env[LD_LIBRARY_PATH] = os.pathsep.join(library_paths)
            logging.debug('prepend ' + local_lib_dir + ' to ' + LD_LIBRARY_PATH)
            logging.debug(LD_LIBRARY_PATH + ' = ' + env[LD_LIBRARY_PATH])


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


def execvp(file, params):
    logging.debug('os.execvp params: ' + str(params))
    sys.stdout.flush()
    sys.stderr.flush()
    os.execvp(file, params)


def exec_tmux(args, tmux_conf):
    params = ('tmux',)
    socket = args.socket
    params += ('-L', socket)
    if args.force256color:
        params += ('-2',)
    tmux_args = args.tmux_args[:]
    # If -- is used to separated between tmux and nmk parameters, don't send it to tmux
    if tmux_args and tmux_args[0] == '--':
        tmux_args.pop(0)
    if is_server_running(socket=socket):
        if tmux_args:
            params += tuple(tmux_args)
        else:
            if 'TMUX' in env and not args.inception:
                logging.error('add --inception to allow nested tmux sessions')
                sys.exit(1)
            params += ('attach',)
    else:
        # start tmux server
        params += ('-f', tmux_conf) + tuple(tmux_args)
    execvp('tmux', params)


def start_login_shell(args, tmux_conf):
    params = ('tmux',)
    if args.force256color:
        params += ('-2',)
    params += ('-f', tmux_conf, '-c', 'zsh -l')
    execvp('tmux', params)


def main():
    args = build_parser().parse_args()
    if args.debug:
        start_pid = get_process_id()
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
    setup_prefer_editor()
    if args.debug:
        end_pid = get_process_id()
        logging.debug('created {0} processes during initialization'.format(end_pid - start_pid - 1))
    tmux_conf = path.relpath(get_tmux_conf(tmux_version, tmux_dir))
    if args.login:
        start_login_shell(args=args, tmux_conf=tmux_conf)
    else:
        exec_tmux(args=args, tmux_conf=tmux_conf)


if __name__ == '__main__':
    main()
