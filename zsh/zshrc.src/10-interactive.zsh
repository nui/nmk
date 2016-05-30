# Aliases and interactive shell configuration
cdd() {
    # Change pwd to directory in which $1 is located
    if [[ ! -e $1 ]]; then
        >&2 print -- '$1 does not exist'
        return 1
    fi
    cd ${1:A:h}
}

cde() {
    # Change current working directory to directory in which $1 is located,
    # and execute the command.
    if [[ ! -x $1 ]]; then
        >&2 print -- '$1 is not executable'
        return 1
    fi
    local prog=${1:A}
    local target_dir=${prog:h}
    pushd -q $target_dir
    shift 1
    $prog "$@"
    popd -q
}

alias cd=' cd'
alias cp='cp --reflink=auto'
alias grep='grep --color=auto'
alias help=run-help
() {
    local -a option
    # Test if --group-directories-first option is available
    ls --group-directories-first --version &> /dev/null && {
        option+=--group-directories-first
    }
    local color_auto
    local color_never
    if [[ $OSTYPE == freebsd* ]]; then
        color_auto='-G'
    else
        # Assume gnu ls
        color_auto='--color=auto'
        color_never='--color=never'
    fi
    alias la=" ls $color_auto $option -lha"
    alias lh=" ls $color_auto $option -lh"
    alias LH=" ls $color_never $option -lhF"
    alias ls="ls $color_auto"
}

type -p readlink &>/dev/null && {
    # rf is shortcut to readlink -f
    # if xclip is present, pipe output to xclip
    if xclip -h &> /dev/null; then
        rf() {
            local fullpath
            fullpath=$(readlink -f $@)
            (( $? == 0 )) && {
                print -- $fullpath | tee >(xclip)
                print -- 'Path is copied to clipboard'
            }
        }
    else
        alias rf=' readlink -f'
    fi
}

# Productive Git aliases
type -p git &>/dev/null && {
    alias gco=' git checkout'
    alias gd=' git diff'
    alias gds=' git diff --staged'
    alias grh=' git reset --hard'
    alias gs=' git status'
    alias gsm=' git merge -s subtree --no-commit --squash'
    # Use alternate screen in git log
    alias lol=" GIT_PAGER='less -+F -+X -c' git log --oneline --decorate --graph --color=auto"
    alias gfr=' git-fetch-rebase'
    alias grrr=' git-reset-to-remote-branch'
}

type -p docker &>/dev/null && {
    alias dkcc=' docker-clear-containers'
    alias dkci=' docker-clear-images'
    alias dkex=' docker-exec'
}

# Vim without my plugins
type -p vi &>/dev/null && alias vi='env -u VIMINIT vi'

# Prefer nvim
type -p nvim &>/dev/null && {
    nvim() {
        # Deactivate python virtual environment before start nvim
        if [[ $(type deactivate) =~ 'shell function$' && -n $VIRTUAL_ENV ]]; then
            (deactivate && command nvim "$@")
        else
            command nvim "$@"
        fi
    }
    alias neo=nvim
}

# Running from command line makes Pycharm inherite all environment variables
# This makes tools installed by npm using nvm work.
type -p pycharm &>/dev/null && alias pycharm=' nohup pycharm &> /dev/null &!'

alias fumount='fusermount -u'

# Fix multimonitor on kubuntu 16.04
if [[ $NMK_DEVELOPMENT == true ]]; then
    alias mm1='xrandr --output DVI-I-1 --off; xrandr --output HDMI1 --off; xrandr --output eDP1 --primary --auto --pos 0x0 --rotate normal; reset-plasma5-panel.py'
    alias mm2='xrandr --output DVI-I-1 --off; xrandr --output HDMI1 --primary --auto --pos 0x0 --rotate normal; xrandr --output eDP1 --auto --pos 1920x0 --rotate normal; reset-plasma5-panel.py'
    alias mm3='xrandr --output DVI-I-1 --auto --pos 0x0 --rotate normal; xrandr --output HDMI1 --primary --auto --pos 1920x0 --rotate normal; xrandr --output eDP1 --auto --pos 3840x0 --rotate normal; reset-plasma5-panel.py'
fi

# apply tmux session environment to running shell
alias ssenv=' eval $(tmux show-environment -s)'

# Disable terminal flow control, so that we can use '^S'
# for history-search-forward.
stty -ixon
