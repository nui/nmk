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
    local -a ls_options
    # Test if --group-directories-first exists
    ls --group-directories-first --version &> /dev/null && {
        ls_options+=--group-directories-first
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
    alias la=" ls $color_auto $ls_options -lha"
    alias lh=" ls $color_auto $ls_options -lh"
    alias LH=" ls $color_never $ls_options -lhF"
    alias ls="ls $color_auto"
}

rf() {
    local -a list
    local _path
    # relative path
    if (( ${+2} )); then
        _path=$(realpath --relative-to=$2 -- $1)
    # absolute path
    else
        _path=${1:A}
    fi
    list=('print -n -- $_path >&1')
    # if running tmux session, load into tmux buffer
    if [[ -n $TMUX ]]; then
        list+='> >(tmux load-buffer -)'
    fi
    # if present and usable, also pipe to xclip
    if (( ${+commands[xclip]} )) && xclip -o &> /dev/null; then
        list+='> >(xclip)'
        list+='> >(xclip -selection clipboard)'
    fi
    eval ${(j: :)list}
}

# Productive Git aliases and functions
(( ${+commands[git]} )) && {
    function git-reset-to-remote-branch {
        git remote update --prune
        git reset --hard $(git for-each-ref --format='%(upstream:short)' $(git symbolic-ref -q HEAD))
        git submodule update
    }
    function grst {
        git tag -d $(git tag)
        git-reset-to-remote-branch
    }
    alias gco=' git checkout'
    alias gd=' git diff'
    alias gds=' git diff --staged'
    alias grh=' git reset --hard'
    alias gs=' git status'
    alias gsm=' git merge -s subtree --no-commit --squash'
    # Use alternate screen in git log
    alias lol=" git log --oneline --decorate --graph --color=auto"
    alias gpr=' git pull --rebase'
    alias grrr=' git-reset-to-remote-branch'
}
export GIT_PAGER='less -+F -+X -c'

(( ${+commands[docker]} )) && {
    alias dkcc=' docker-clear-containers'
    alias dkci=' docker-clear-images'
}

# vi = Vim without my plugins
#   The use of function keyword in function declaration
#   is to prevent vi get expanded to vim on some system
#   that alias vi=vim
(( ${+commands[vi]} )) && function vi {
    local VIMINIT=
    command vi "$@"
}
# unalias vi, because it can override previous vi function
(( ${+aliases[vi]} )) && unalias vi

# Prefer nvim
(( ${+commands[nvim]} )) && {
    function nvim {
        # Deactivate python virtual environment before start nvim
        if [[ -n $VIRTUAL_ENV ]]; then
            if (( ${+functions[deactivate]} )); then
                (deactivate && command nvim "$@")
            elif (( ${+commands[pyenv]} )); then
                if [[ ${$(pyenv commands)[(r)deactivate]} == deactivate ]]; then
                    (pyenv deactivate && command nvim "$@")
                fi
            fi
        else
            command nvim "$@"
        fi
    }
    alias neo=nvim
}

# Running from command line makes Pycharm inherite all environment variables
# This makes tools installed by npm using nvm work.
(( ${+commands[pycharm]} )) && alias pycharm=' nohup pycharm &> /dev/null &!'

# Fix multimonitor on kubuntu 16.04
if [[ $NMK_DEVELOPMENT == true ]]; then
    alias mm1='xrandr --output HDMI1 --off; xrandr --output eDP1 --primary --auto --pos 0x0 --rotate normal; reset-plasma5-panel.py'
    alias mm2='xrandr --output eDP1 --auto --pos 0x0 --rotate normal; xrandr --output HDMI1 --primary --auto --pos 1920x-100 --rotate normal; reset-plasma5-panel.py'
    # alias mm1='xrandr --output DVI-I-1 --off; xrandr --output HDMI1 --off; xrandr --output eDP1 --primary --auto --pos 0x0 --rotate normal; reset-plasma5-panel.py'
    # alias mm2='xrandr --output DVI-I-1 --off; xrandr --output eDP1 --auto --pos 0x0 --rotate normal; xrandr --output HDMI1 --primary --auto --pos 1920x-100 --rotate normal; reset-plasma5-panel.py'
    # alias mm3='xrandr --output DVI-I-1 --auto --pos 0x0 --rotate normal; xrandr --output HDMI1 --primary --auto --pos 1920x0 --rotate normal; xrandr --output eDP1 --auto --pos 3840x0 --rotate normal; reset-plasma5-panel.py'
fi

# apply tmux session environment to running shell
alias ssenv=' eval $(tmux show-environment -s)'

# Disable terminal flow control, so that we can use '^S'
# for history-search-forward.
unsetopt FLOW_CONTROL
