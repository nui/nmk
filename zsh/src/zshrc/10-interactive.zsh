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
    list+='; print' # add newline to output
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
    alias rebase='git rebase -i'
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
        if [[ -n $PYENV_VIRTUAL_ENV ]]; then
            (pyenv deactivate && command nvim "$@")
        elif [[ -n $VIRTUAL_ENV ]] && (( ${+functions[deactivate]} )); then
            (deactivate && command nvim "$@")
        else
            command nvim "$@"
        fi
    }
    alias neo=nvim
}

# apply tmux session environment to running shell
alias ssenv=' eval $(tmux show-environment -s)'

# fix tmux and zsh corrupt after cat binary file
# ref: https://unix.stackexchange.com/a/253369
reset() {
    stty sane
    printf '\033k\033\\\033]2;\007'
    tput reset
    if [[ -n $TMUX ]]; then
        tmux set-window-option automatic-rename on
        tmux refresh
    fi
}

# if TMOUT is set on some environment, extend it to 24 hours
if [[ $TMOUT = <-> ]] && (( $TMOUT <= 24*3600 )); then
    export TMOUT=$(( 24*3600 ))
fi

# Disable terminal flow control, so that we can use '^S'
# for history-search-forward.
unsetopt FLOW_CONTROL
