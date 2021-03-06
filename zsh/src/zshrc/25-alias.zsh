# Aliases and interactive shell configuration
autoload -Uz cdd
autoload -Uz cde

alias cd=' cd'
[[ $OSTYPE == linux* ]] && alias cp='cp --reflink=auto'
alias grep='grep --color=auto'
alias help=run-help
() {
    local -a ls_options
    local color

    local prog=ls
    local version=gnu

    if ((${+commands[lsd]})); then
        ls_options+=(--group-dirs first)
        if [[ $TERMINAL_EMULATOR = JetBrains-JediTerm ]]; then
            color="--color=never"
        fi
        alias la=" lsd -lah $color $ls_options"
        alias lh="lsd -lh $color $ls_options"
        alias ls="lsd $color $ls_options"
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
            color='--color=auto'
        else
            color='-G'
        fi

        alias la=" command $prog $color $ls_options -lha"
        alias lh="command $prog $color $ls_options -lh"
        alias ls="command $prog $color"
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


[[ $OSTYPE == darwin* ]] && () {
    jetbrains-toolbox() {
        local label
        local launch_agent_dir
        local script_dir
        local target_job
        label=nmk.jetbrains.toolbox.launcher
        launch_agent_dir=~/Library/LaunchAgents
        script_dir=$NMK_HOME/etc/macos
        target_job=$launch_agent_dir/${label}.plist
        if [[ ! -e $target_job ]]; then
            cp $script_dir/${label}.plist $target_job
            print "Log out and log back in to load jetbrains toolbox"
        else
            launchctl start $label
        fi
    }
}

