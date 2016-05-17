#!/usr/bin/python2
import argparse
import logging
import os
import subprocess
import sys
from exceptions import IOError

logging.basicConfig(format='%(levelname)s:%(message)s',
                    level=logging.DEBUG)

UNICODE_NAME = 'en_US.UTF-8'


def is_exec(path):
    return os.path.isfile(path) and os.access(path, os.X_OK)


def which(program):
    head, _ = os.path.split(program)
    # if 'program' is absolute or relative path
    if head and is_exec(program):
        return program
    # if 'program' is just a name, for example, zsh
    else:
        # return absolute path to 'program'
        for d in os.environ["PATH"].split(os.pathsep):
            d = d.strip('"')
            f = os.path.join(d, program)
            if is_exec(f):
                return f
        return None


def build_parser():
    parser = argparse.ArgumentParser()
    parser.add_argument('-2',
                        dest='force256color',
                        action='store_true',
                        default=False,
                        help='force 256 colors terminal')
    parser.add_argument('-L', '--socket',
                        dest='socket',
                        default='nmk',
                        help='set tmux socket name')
    parser.add_argument('-u', '--unicode',
                        dest='unicode',
                        action='store_true',
                        default=False,
                        help='export LANG={}'.format(UNICODE_NAME))
    parser.add_argument('--force-unicode',
                        dest='force_unicode',
                        action='store_true',
                        default=False,
                        help='export LC_ALL={}'.format(UNICODE_NAME))
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
    parser.add_argument('-I', '--ignore-local',
                        dest='local_config',
                        action='store_false',
                        default=True,
                        help='ignore local configuration')
    return parser


def parse_cgroup(cgroup_file):
    with open(cgroup_file) as f:
        for line in f:
            hierarchy_id, subsystems, control_group = line.strip().split(':')
            yield control_group


def is_inside_docker():
    cgroup_file = '/proc/1/cgroup'
    try:
        control_groups = parse_cgroup(cgroup_file)
        in_docker = all((g != '/' for g in control_groups))
    except IOError:
        in_docker = False
        logging.error("Couldn't read {}".format(cgroup_file))
    return in_docker


def check_dependencies():
    for prog in ('tmux', 'zsh'):
        if not which(prog):
            logging.error('{} command not found'.format(prog))
            sys.exit(1)


def setup_terminal(tmux_dir, args, env):
    use_256color = any((
        args.force256color,
        args.autofix and is_inside_docker(),
        env.get('TERM') in ('gnome-256color', 'screen-256color', 'xterm-256color'),
        env.get('COLORTERM') in ('gnome-terminal', 'rxvt-xpm', 'xfce4-terminal'),
        ))
    if use_256color:
        color_profile = '256color.conf'
        terminal = 'screen-256color'
    else:
        color_profile = '8color.conf'
        terminal = 'screen'
    env['NMK_TMUX_COLOR_PROFILE'] = os.path.join(tmux_dir, color_profile)
    env['NMK_TMUX_DEFAULT_TERMINAL'] = terminal


def setup_environment(nmk_dir, args, env):
    initvim = os.path.join(nmk_dir, 'vim/init.vim')
    zdotdir = os.path.join(nmk_dir, 'zsh')

    env['NMK_AUTOLOAD'] = str(args.autoload).lower()
    env['NMK_DIR'] = nmk_dir
    env['NMK_IGNORE_LOCAL'] = str(not args.local_config).lower()
    env['NMK_TMUX_DEFAULT_SHELL'] = which('zsh')
    env['NMK_TMUX_DETACH_ON_DESTROY'] = args.detach_on_destroy
    env['VIMINIT'] = 'source {}'.format(initvim.replace(' ', r'\ '))
    env['ZDOTDIR'] = zdotdir

    if 'VIRTUAL_ENV' in env:
        del env['VIRTUAL_ENV']

    if args.unicode or (args.autofix and 'LANG' not in env):
        env['LANG'] = UNICODE_NAME

    if args.force_unicode:
        env['LC_ALL'] = UNICODE_NAME


def setup_prefer_editor(env):
    prefer_editors = ('nvim', 'vim')
    if 'EDITOR' not in env:
        for prog in prefer_editors:
            if which(prog):
                env['EDITOR'] = prog
                break


def add_local_library(env, nmk_dir):
    local_lib_dir = os.path.join(nmk_dir, 'local', 'lib')
    if os.path.isdir(local_lib_dir):
        library_path = env.get('LD_LIBRARY_PATH', '')
        if len(library_path) > 1:
            library_paths = library_path.split(':')
        else:
            library_paths = []
        library_paths.insert(0, local_lib_dir)
        new_library_path = ':'.join(library_paths)
        env['LD_LIBRARY_PATH'] = new_library_path


def manage_path_env(env, nmk_dir):
    """
    Clean up PATH.
      - <virtualenv>/bin is removed
      - prepend NMK_DIR
      - remove duplicate directory
    """
    def is_virtualenv(dir):
        scripts = (os.path.join(dir, f) for f in ('activate', 'python'))
        return all((os.path.exists(p) for p in scripts))
    dirs = [d for d in env['PATH'].split(os.pathsep) if not is_virtualenv(d)]
    dirs.insert(0, os.path.join(nmk_dir, 'bin'))
    dirs.insert(0, os.path.join(nmk_dir, 'local', 'bin'))
    unique = []
    for d in dirs:
        if d not in unique:
            unique.append(d)
    env['PATH'] = os.pathsep.join(unique)


def find_tmux_version():
    output = subprocess.check_output(('tmux', '-V')).strip()
    return output.split()[1]


def is_socket_exist(socket):
    devnull = open(os.devnull, 'w')
    return 0 == subprocess.call(('tmux', '-L', socket, 'server-info'),
                                stdout=devnull,
                                stderr=devnull)


def exec_tmux(tmux_dir, args, unknown):
    version = find_tmux_version()
    conf = os.path.join(tmux_dir, '{}.conf'.format(version))
    if not os.path.exists(conf):
        logging.error('tmux {} is unsupported'.format(version))
        sys.exit(1)

    cmd = 'tmux'
    params = (cmd,)
    # Use default socket unless socket name is specified.
    socket = args.socket
    params += ('-L', socket)
    if args.force256color:
        params += ('-2',)
    if is_socket_exist(socket=socket):
        if len(unknown) > 0:
            params += tuple(unknown)
        else:
            params += ('attach',)
    else:
        # start tmux server
        params += ('-f', conf) + tuple(unknown)
    sys.stdout.flush()
    os.execv(which(cmd), params)


def main():
    # check_dependencies()
    (args, unknown) = build_parser().parse_known_args()
    nmk_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    tmux_dir = os.path.join(nmk_dir, 'tmux')
    setup_terminal(tmux_dir=tmux_dir, args=args, env=os.environ)
    setup_environment(nmk_dir=nmk_dir, args=args, env=os.environ)
    setup_prefer_editor(env=os.environ)
    add_local_library(env=os.environ, nmk_dir=nmk_dir)
    manage_path_env(env=os.environ, nmk_dir=nmk_dir)
    exec_tmux(tmux_dir=tmux_dir, args=args, unknown=unknown)

if __name__ == '__main__':
    main()
