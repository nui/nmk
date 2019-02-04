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
    local color_never
    # Detect ls version using --group-directories-first option
    if ls --group-directories-first &> /dev/null; then
        ls_options+=--group-directories-first
        color_auto='--color=auto'
        color_never='--color=never'
    else
        color_auto='-G'
    fi
    alias la=" \ls $color_auto $ls_options -lha"
    alias lh=" \ls $color_auto $ls_options -lh"
    alias LH=" \ls $color_never $ls_options -lhF"
    alias ls="\ls $color_auto"
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

