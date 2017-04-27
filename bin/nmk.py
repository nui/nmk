"""
Run on Python 2.6.6 and later
"""

import logging
import os
import subprocess
import sys
import tempfile

import argparse
import six

UNICODE_NAME = 'en_US.UTF-8'
ENV = os.environ


def build_parser():
    parser = argparse.ArgumentParser()
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
    parser.add_argument('--no-autoload',
                        dest='autoload',
                        action='store_false',
                        default=True,
                        help='do not detect and load common development tools')
    parser.add_argument('-d', '--debug',
                        dest='debug',
                        action='store_true',
                        default=False,
                        help='print debug log')
    parser.add_argument('tmux_args', nargs=argparse.REMAINDER)
    return parser


def is_exec(path):
    return os.path.isfile(path) and os.access(path, os.X_OK)


def whence(program):
    """
    :return: path to program or None
    """
    head, _ = os.path.split(program)
    # if 'program' is absolute or relative path
    if head and is_exec(program):
        return program
    # if 'program' is just a name, for example, zsh
    else:
        # return absolute path to 'program'
        for d in os.environ["PATH"].split(os.pathsep):
            f = os.path.join(d, program)
            if is_exec(f):
                return f
        return None


def filter_duplicate_values(collection):
    unique = []
    for item in collection:
        if item not in unique:
            unique.append(item)
    return unique


def check_output(args):
    """
    Replacement of subprocess.check_output in python2.6
    """
    stdout = tempfile.TemporaryFile()
    subprocess.call(args, stdout=stdout)
    stdout.flush()
    stdout.seek(0)
    return stdout.read()


def setup_logging(debug):
    level = logging.DEBUG if debug else logging.WARNING
    logging.basicConfig(format='%(levelname)s:%(message)s',
                        level=level)


def python_info():
    logging.debug('use python from ' + sys.executable)
    logging.debug('python ' + sys.version)


def is_virtualenv_bin(path):
    virtualenv_files = (os.path.join(path, name) for name in ('activate', 'python'))
    return all((os.path.exists(p) for p in virtualenv_files))


def setup_path(nmk_dir):
    """
    Setup PATH environment.
      - get rid of <virtualenv>/bin
      - prepend NMK_DIR/bin and NMK_DIR/local/bin
      - remove duplicate paths
    """
    paths = [d for d in ENV['PATH'].split(os.pathsep) if not is_virtualenv_bin(d)]
    paths.insert(0, os.path.join(nmk_dir, 'local', 'bin'))
    paths.insert(0, os.path.join(nmk_dir, 'bin'))
    unique_paths = filter_duplicate_values(paths)
    ENV['PATH'] = os.pathsep.join(unique_paths)
    logging.debug('PATH:' + ENV['PATH'])
    for p in unique_paths:
        logging.debug('PATH:' + p)


def check_dependencies():
    for prog in ('tmux', 'zsh'):
        if not whence(prog):
            logging.error('{0} not found'.format(prog))
            sys.exit(1)


def parse_cgroup(cgroup_file):
    with open(cgroup_file) as f:
        for line in f:
            hierarchy_id, subsystems, control_group = line.strip().split(':')
            yield control_group


def is_inside_docker():
    cgroup_file = '/proc/1/cgroup'
    if os.path.exists(cgroup_file):
        control_groups = parse_cgroup(cgroup_file)
        in_docker = all((g != '/' for g in control_groups))
    else:
        in_docker = False
        logging.error("Couldn't read {0}".format(cgroup_file))
    return in_docker


def setup_terminal(args):
    support_256color = any((
        args.force256color,
        ENV.get('TERM') in ('cygwin', 'gnome-256color', 'putty', 'screen-256color', 'xterm-256color'),
        ENV.get('COLORTERM') in ('gnome-terminal', 'rxvt-xpm', 'xfce4-terminal'),
        args.autofix and is_inside_docker(),
    ))

    use_256color = not args.force8color and support_256color
    if use_256color:
        terminal = 'screen-256color'
    else:
        terminal = 'screen'
    ENV['NMK_TMUX_DEFAULT_TERMINAL'] = terminal
    ENV['NMK_TMUX_256_COLOR'] = "1" if use_256color else "0"


def setup_environment(args, nmk_dir):
    initvim = os.path.join(nmk_dir, 'vim/init.vim')
    zdotdir = os.path.join(nmk_dir, 'zsh')

    ENV['NMK_AUTOLOAD'] = str(args.autoload).lower()
    ENV['NMK_DIR'] = nmk_dir
    ENV['NMK_TMUX_DEFAULT_SHELL'] = whence('zsh')
    ENV['NMK_TMUX_DETACH_ON_DESTROY'] = args.detach_on_destroy
    ENV['NMK_TMUX_HISTORY'] = os.path.join(nmk_dir, 'tmux', '.tmux_history')
    ENV['VIMINIT'] = 'source {0}'.format(initvim.replace(' ', r'\ '))
    ENV['ZDOTDIR'] = zdotdir

    if 'VIRTUAL_ENV' in ENV:
        del ENV['VIRTUAL_ENV']
        logging.debug('unset VIRTUAL_ENV')

    if args.unicode or (args.autofix and 'LANG' not in ENV):
        ENV['LANG'] = UNICODE_NAME
        logging.debug('set LANG = ' + UNICODE_NAME)

    if args.force_unicode:
        ENV['LC_ALL'] = UNICODE_NAME
        logging.debug('set LC_ALL = ' + UNICODE_NAME)


def setup_prefer_editor():
    prefer_editors = ('nvim', 'vim')
    if 'EDITOR' not in ENV:
        for prog in prefer_editors:
            if whence(prog):
                ENV['EDITOR'] = prog
                logging.debug('set EDITOR = ' + prog)
                break


def add_local_library(nmk_dir):
    LD_LIBRARY_PATH = 'LD_LIBRARY_PATH'

    local_lib_dir = os.path.join(nmk_dir, 'local', 'lib')
    if os.path.isdir(local_lib_dir):
        library_path = ENV.get(LD_LIBRARY_PATH)
        library_paths = library_path.split(os.pathsep) if library_path else []
        library_paths.insert(0, local_lib_dir)
        ENV[LD_LIBRARY_PATH] = os.pathsep.join(library_paths)
        logging.debug('prepend ' + local_lib_dir + ' to ' + LD_LIBRARY_PATH)
        logging.debug(LD_LIBRARY_PATH + ' = ' + ENV[LD_LIBRARY_PATH])


def find_tmux_version():
    output = check_output(('tmux', '-V'))
    if isinstance(output, six.binary_type):
        output = output.decode()
    return output.split()[1]


def get_tmux_conf(tmux_dir):
    version = find_tmux_version()
    conf = os.path.join(tmux_dir, '{0}.conf'.format(version))
    if not os.path.exists(conf):
        logging.error('tmux {0} is unsupported'.format(version))
        sys.exit(1)
    return conf


def is_socket_exist(socket):
    devnull = open(os.devnull, 'w')
    exists = 0 == subprocess.call(('tmux', '-L', socket, 'list-sessions'),
                                  stdout=devnull,
                                  stderr=devnull)
    status = "does exists" if exists else "doesn't exists"
    logging.debug("socket '{0}' {1}".format(socket, status))
    return exists


def exec_tmux(args, tmux_dir):
    conf = os.path.relpath(get_tmux_conf(tmux_dir))

    params = ('tmux',)
    # Use default socket unless socket name is specified.
    socket = args.socket
    params += ('-L', socket)
    if args.force256color:
        params += ('-2',)
    tmux_args = args.tmux_args[:]
    # If -- is used to separated between tmux and nmk parameters, don't send it to tmux
    if tmux_args and tmux_args[0] == '--':
        tmux_args.pop(0)
    if is_socket_exist(socket=socket):
        if tmux_args:
            params += tuple(tmux_args)
        else:
            params += ('attach',)
    else:
        # start tmux server
        params += ('-f', conf) + tuple(tmux_args)
    logging.debug('os.execvp params: ' + str(params))
    sys.stdout.flush()
    os.execvp('tmux', params)


def main():
    args = build_parser().parse_args()
    setup_logging(debug=args.debug)
    python_info()
    nmk_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    tmux_dir = os.path.join(nmk_dir, 'tmux')
    setup_path(nmk_dir=nmk_dir)
    check_dependencies()
    setup_terminal(args=args)
    setup_environment(args=args, nmk_dir=nmk_dir)
    setup_prefer_editor()
    add_local_library(nmk_dir=nmk_dir)
    exec_tmux(args=args, tmux_dir=tmux_dir)


if __name__ == '__main__':
    main()
