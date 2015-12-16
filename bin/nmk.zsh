#!/usr/bin/env zsh

# Constant
_UNICODE_NAME='en_US.UTF-8'

# Parse options {{{
usage() {
cat <<- EOU
Usage: nmk [OPTIONS] [TMUX_COMMANDS [TMUX_OPTIONS...]]

Options
    -2                        force 256 colors terminal
    -L SOCKET_NAME            set tmux socket name
    -u, --unicode             export LANG=$_UNICODE_NAME
        --force-unicode       export LC_ALL=$_UNICODE_NAME
        --detach-on-destroy   detach the client when the session is destroyed
        --no-autofix          disable automatically fix
        --no-autoload         do not detect and load common development tools
    -I, --ignore-local        ignore local configuration
    -h, --help                print this help message
EOU
}

_OPTIONS=(
    detach-on-destroy
    force-unicode
    help
    ignore-local
    no-autofix
    no-autoload
    unicode
)

if ! _TEMP=$(POSIXLY_CORRECT=true getopt -q -o 2hIL:u --long ${(j:,:)_OPTIONS} -- "$@"); then
    # exit if error
    usage
    exit 1
fi

eval set -- $_TEMP

# Variables
NMK_AUTOLOAD=
NMK_IGNORE_LOCAL=
NMK_TMUX_DETACH_ON_DESTROY=off
_256COLOR=false
_AUTOFIX=true
_SOCKET_NAME=nmk

while true; do
    case $1 in
        -2 ) _256COLOR=true; shift ;;
        -L ) _SOCKET_NAME=$2; shift 2 ;;
        -u | --unicode ) export LANG=$_UNICODE_NAME; shift ;;
        --detach-on-destroy ) NMK_TMUX_DETACH_ON_DESTROY=on; shift ;;
        --force-unicode ) export LC_ALL=$_UNICODE_NAME; shift ;;
        --no-autofix ) _AUTOFIX=false; shift ;;
        --no-autoload ) NMK_AUTOLOAD=false; shift ;;
        -I | --ignore-local ) NMK_IGNORE_LOCAL=true; shift ;;
        -h | --help ) usage; exit 0 ;;
        -- ) shift; break ;;
    esac
done
# }}}

# Functions {{{
is_command_exist() {
    hash "$1" &> /dev/null
}

is_element() {
    [[ -n $1 ]] && [[ ${${@:2}[(r)$1]} == $1 ]]
}

# see http://stackoverflow.com/questions/20010199/determining-if-a-process-runs-inside-lxc-docker
is_inside_docker() {
    local -a fields
    local in_docker=false
    for line (${(f)"$(</proc/1/cgroup)"}) {
        fields=("${(@s/:/)line}")
        [[ $fields[3] != / ]] && {
            in_docker=true
            break
        }
    }
    [[ $in_docker == true ]]
}
# End function }}}

# set up Nmk directory
NMK_DIR=${0:A:h:h}

# check dependencies
() {
    local -a dependencies

    dependencies=(tmux zsh)
    for prog in $dependencies; do
        is_command_exist $prog || {
            >&2 echo "Error: $prog: command not found"
            exit 1
        }
    done
    getopt --test &> /dev/null
    if [[ $? -ne 4 ]]; then
        >&2 echo 'Gnu getopt is required'
        exit 1
    fi
}

# set LANG to unicode if it's unset
# especially useful when running docker interactive mode
[[ $_AUTOFIX == true ]] && [[ -z $LANG ]] && export LANG=$_UNICODE_NAME

# setup TERM variable
() {
    # check if terminal supports 256 colors

    # case 1, by terminal type
    if is_element $TERM 'gnome-256color' 'screen-256color' 'xterm-256color' ||
        # case 2, by $COLORTERM variable
        is_element $COLORTERM 'gnome-terminal' 'rxvt-xpm' 'xfce4-terminal' ||
        # force 256 colors support in docker containers
        { [[ $_AUTOFIX == true ]] && is_inside_docker } then
            _256COLOR=true
    fi

    if [[ $_256COLOR == true ]]; then
        NMK_TMUX_COLOR_PROFILE="$NMK_DIR/tmux/256color.conf"
        NMK_TMUX_DEFAULT_TERMINAL='screen-256color'
    else
        NMK_TMUX_COLOR_PROFILE="$NMK_DIR/tmux/8color.conf"
        NMK_TMUX_DEFAULT_TERMINAL='screen'
    fi
}

# setup shell
() {
    # always use zsh
    NMK_TMUX_DEFAULT_SHELL=$(command -v zsh)
}

# setup environment variables
() {
    # always export
    export NMK_DIR
    export VIMINIT="source ${NMK_DIR:q}/vim/init.vim"
    export ZDOTDIR="$NMK_DIR/zsh"
    # export if length is nonzero
    local -a envs
    envs=(NMK_AUTOLOAD
          NMK_IGNORE_LOCAL)
    for env in $envs; do
        [[ -n ${(P)env} ]] && export $env
    done
    # temporary environment variables used during tmux initialization
    # its will be unset by either zsh or tmux
    export NMK_TMUX_COLOR_PROFILE
    export NMK_TMUX_DEFAULT_SHELL
    export NMK_TMUX_DEFAULT_TERMINAL
    export NMK_TMUX_DETACH_ON_DESTROY

    # prefer vim over nvim
    is_command_exist nvim && export EDITOR='nvim'
    is_command_exist vim && export EDITOR='vim'
    # prepend $NMK_DIR/bin to $PATH
    path[1,0]=$NMK_DIR/bin
    # merge duplicated path
    typeset -Ug path
}

# execute tmux
() {
    local -a params
    local version=${$(tmux -V)[2]}

    local tmux_conf="$NMK_DIR/tmux/${version}.conf"
    if [[ ! -f $tmux_conf ]]; then
        >&2 echo "Unsupported tmux version $version"
        exit 1
    fi
    # Use default socket unless socket name is specified.
    params+=(-L $_SOCKET_NAME)
    [[ $_256COLOR == true ]] && params+=(-2)
    if tmux -L $_SOCKET_NAME server-info &> /dev/null; then
        if [[ $# -gt 0 ]]; then
            # pass remaining arguments to tmux
            params+=("$@")
        # attach to sessions
        else
            params+=(attach)
        fi
    # start tmux server
    else
        params+=(-f $tmux_conf "$@")
    fi
    exec tmux "${(@)params}"
} "$@"

# vim: ft=zsh sw=4 sts=4 ts=4
