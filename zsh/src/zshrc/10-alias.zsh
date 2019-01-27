# Aliases and interactive shell configuration
cdd() {
    # Change pwd to directory in which $1 is located
    if [[ ! -e $1 ]]; then
        >&2 print -- '$1 does not exist'
        return 1
    fi
    cd ${1:a:h}
}

cde() {
    # Change current working directory to directory in which $1 is located,
    # and execute the command.
    if [[ ! -x $1 ]]; then
        >&2 print -- '$1 is not executable'
        return 1
    fi
    local prog=${1:a}
    local target_dir=${prog:h}
    pushd -q $target_dir
    shift 1
    $prog "$@"
    popd -q
}

alias cd=' cd'
[[ $OSTYPE == linux* ]] && alias cp='cp --reflink=auto'
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
    case $OSTYPE in
        darwin* | freebsd*)
            color_auto='-G'
            ;;
        linux*)
            color_auto='--color=auto'
            color_never='--color=never'
            ;;
    esac
    alias la=" \ls $color_auto $ls_options -lha"
    alias lh=" \ls $color_auto $ls_options -lh"
    alias LH=" \ls $color_never $ls_options -lhF"
    alias ls="\ls $color_auto"
}

autoload -Uz rf

# Productive Git aliases and functions
(( ${+commands[git]} )) && {
    autoload -Uz git-reset-to-remote-branch
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

# vi = Vim without plugins
(( ${+commands[vi]} )) && {
    alias vi='env -u VIMINIT vi'
}

[[ -n $EDITOR ]] && alias neo=$EDITOR

# apply tmux session environment to running shell
alias ssenv=' eval $(tmux show-environment -s)'

