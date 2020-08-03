# Aliases and interactive shell configuration
autoload -Uz cdd
autoload -Uz cde

alias cd=' cd'
[[ $OSTYPE == linux* ]] && alias cp='cp --reflink=auto'
alias grep='grep --color=auto'
alias help=run-help
() {
    local -a ls_options
    local color_auto

    local prog=ls
    local version=gnu

    if ((${+commands[lsd]})); then
        ls_options+=(--group-dirs first)
        alias la=" lsd -lah $ls_options"
        alias lh="lsd -lh $ls_options"
        alias ls="lsd"
    else
        case $OSTYPE in
            linux*) ;;
            darwin*)
                if (( ${+commands[gls]} )); then
                    prog=gls
                else
                    version=bsd
                fi
                ;;
            freebsd*) version=bsd ;;
        esac

        if [[ $version == gnu ]]; then
            ls_options+=--group-directories-first
            color_auto='--color=auto'
        else
            color_auto='-G'
        fi

        alias la=" command $prog $color_auto $ls_options -lha"
        alias lh="command $prog $color_auto $ls_options -lh"
        alias ls="command $prog $color_auto"
    fi
}

autoload -Uz rf

# Productive Git aliases and functions
(( ${+commands[git]} )) && {
    autoload -Uz git-reset-to-remote-branch
    autoload -Uz grst
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

# vi = Vim without plugins
(( ${+commands[vi]} )) && {
    alias vi='env -u VIMINIT vi'
}

[[ -n $EDITOR ]] && alias neo=$EDITOR

# apply tmux session environment to running shell
alias ssenv=' eval $(tmux show-environment -s)'

# reset nvidia gpu
alias gpu-reload="sudo rmmod nvidia_uvm ; sudo modprobe nvidia_uvm"

