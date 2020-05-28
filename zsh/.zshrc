() {
    local file
    for file ($ZDOTDIR/zshrc.pre.d/*.zsh(N)) source $file
}
######################### START JETBRAINS TERMINAL HACK ########################
# Fix jetbrains terminal incorrectly set $0 to jetbrains-toolbox
#
# This bug affect linux only
# Reproduce steps
#   1) Start jetbrains-toolbox
#   2) Start any IDE
#   3) Open terminal panel
#   4) Run any command, $0 will be set to jetbrains-toolbox absolute path
#
# If we get back to this again, but problem still exist, let report to jetbrains.
#
if [[ -n $TOOLBOX_VERSION && $TERMINAL_EMULATOR == "JetBrains-JediTerm" ]]; then
    # This bug affect linux only
    if [[ $OSTYPE == linux* ]]; then
        case $JETBRAINS_TERMINAL_HACK in
            "")
                export JETBRAINS_TERMINAL_HACK=1
                exec $SHELL
                ;;
            1)
                export JETBRAINS_TERMINAL_HACK=2
                if [[ $0 == *jetbrains-toolbox ]]; then
                    exec $SHELL
                else
                    echo "Jetbrains terminal hack is no longer need"
                fi
                ;;
            2 | *)
                ;;
        esac
    fi
fi
########################## END JETBRAINS TERMINAL HACK #########################
autoload -Uz edit-command-line && zle -N edit-command-line
autoload -Uz promptinit && promptinit
autoload -Uz async && async

setopt AUTO_PUSHD
setopt DVORAK
setopt EXTENDED_GLOB
setopt HIST_IGNORE_ALL_DUPS
setopt HIST_IGNORE_SPACE
setopt PUSHD_MINUS
setopt SHARE_HISTORY

# Release ^S for use in history-incremental-pattern-search-forward
unsetopt FLOW_CONTROL
stty -ixon # vim in remote ssh connection need this

HISTFILE="${ZDOTDIR}/.zsh_history"
HISTSIZE=2500
SAVEHIST=$HISTSIZE
autoload -Uz compinit && compinit
zstyle ':completion:*' auto-description 'specify: %d'
zstyle ':completion:*' completer _expand _complete _correct _approximate
zstyle ':completion:*' format 'Completing %d'
zstyle ':completion:*' group-name ''
zstyle ':completion:*' menu select=2
zstyle ':completion:*' list-colors ''
zstyle ':completion:*' list-prompt %SAt %p: Hit TAB for more, or the character to insert%s
zstyle ':completion:*' matcher-list '' 'm:{a-z}={A-Z}' 'm:{a-zA-Z}={A-Za-z}' 'r:|[._-]=* r:|=* l:|=*'
zstyle ':completion:*' menu select=long
zstyle ':completion:*' select-prompt %SScrolling active: current selection at %p%s
zstyle ':completion:*' verbose true
zstyle ':completion:*:*:kill:*:processes' list-colors '=(#b) #([0-9]#)*=0=01;31'
zstyle ':completion:*:kill:*' command 'ps -a -o tty,pid,%cpu,cmd k %cpu'
() {
    local cmd
    for cmd (dircolors gdircolors) {
        (( ${+commands[$cmd]} )) && {
            eval "$($cmd -b)"
            zstyle ':completion:*:default' list-colors ${(s.:.)LS_COLORS}
            break
        }
    }
}
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

    local prog=ls
    local version=gnu

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
        color_never='--color=never'
    else
        color_auto='-G'
    fi

    alias la=" command $prog $color_auto $ls_options -lha"
    alias lh="command $prog $color_auto $ls_options -lh"
    alias LH=" command $prog $color_never $ls_options -lhF"
    alias ls="command $prog $color_auto"
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

() {
    # see /etc/zsh/zshrc
    local -A key
    key=(
        BackSpace  "${terminfo[kbs]}"
        Home       "${terminfo[khome]}"
        End        "${terminfo[kend]}"
        Insert     "${terminfo[kich1]}"
        Delete     "${terminfo[kdch1]}"
        Up         "${terminfo[kcuu1]}"
        Down       "${terminfo[kcud1]}"
        Left       "${terminfo[kcub1]}"
        Right      "${terminfo[kcuf1]}"
        PageUp     "${terminfo[kpp]}"
        PageDown   "${terminfo[knp]}"
        CtrlL      "^L"
        CtrlR      "^R"
        CtrlS      "^S"
        CtrlZ      "^Z"
    )

    bind2maps() {
        local i sequence widget
        local -a maps

        while [[ "$1" != "--" ]]; do
            maps+=( "$1" )
            shift
        done
        shift

        sequence="${key[$1]}"
        widget="$2"

        [[ -z "$sequence" ]] && return 1

        for i in "${maps[@]}"; do
            bindkey -M "$i" "$sequence" "$widget"
        done
    }

    # use emacs keybindings
    bindkey -e

    if [[ -n $NMK_TMUX_VERSION ]]; then
        # PageUp to enter copy mode
        _nmk-tmux-copy-mode() tmux copy-mode -eu
        zle -N _nmk-tmux-copy-mode
        bind2maps emacs         -- PageUp     _nmk-tmux-copy-mode

        # ^L to clear tmux history
        autoload -Uz nmk-tmux-clear-history && zle -N nmk-tmux-clear-history
        bind2maps emacs         -- CtrlL      nmk-tmux-clear-history
    else
        bind2maps emacs         -- PageUp     redisplay
    fi
    # PageDown do nothing
    bind2maps emacs             -- PageDown   redisplay
    # Search backwards and forwards with a pattern
    bind2maps emacs -- CtrlR history-incremental-pattern-search-backward
    bind2maps emacs -- CtrlS history-incremental-pattern-search-forward

    bindkey '^X^E' edit-command-line
    autoload -Uz fancy-ctrl-z && zle -N fancy-ctrl-z
    bind2maps emacs -- CtrlZ fancy-ctrl-z

    # Fix Home, End, and Delete Key in build-from-source tmux
    bind2maps emacs -- Home     beginning-of-line
    bind2maps emacs -- End      end-of-line
    bind2maps emacs -- Delete   delete-char

    unfunction bind2maps
}
autoload -Uz reset
autoload -Uz update-nmkpkg

() {
    local min_tmout=$(( 24*3600 ))
    # if TMOUT is set on some environment, extend it to 24 hours
    [[ $TMOUT = <-> ]] && (( $TMOUT <= $min_tmout )) && export TMOUT=$(( $min_tmout ))
}

# Don't display git branch symbol if terminal does not support 256 colors
(( ${+commands[tput]} )) && (( $(command tput colors) < 256 )) && horizontal_branch_symbol=

prompt horizontal

# Hide user and host in prompt if NMK_DEVELOPMENT is true by default,
# this is not apply to zsh in ssh session
[[ $NMK_DEVELOPMENT == true && -z $SSH_TTY ]] && horizontal[userhost]=0

# Change prompt color to yellow in remote session
[[ -n $SSH_TTY ]] && horizontal[base_color]=magenta
_nmk_precmd_functions=()
_nmk_preexec_functions=()

_nmk-kubectl-precmd() {
    if [[ -n $KUBECTL_CONTEXT ]]; then
        alias kubectl="kubectl --context=$KUBECTL_CONTEXT"
    fi
}

_nmk-kubectl-preexec() {
    if [[ -n $KUBECTL_CONTEXT ]]; then
        unalias kubectl
    fi
}

_nmk_update_ssh_socket_last_check=$EPOCHSECONDS
_nmk-update-ssh-socket() {
    [[ -n $SSH_AUTH_SOCK && ! -S $SSH_AUTH_SOCK ]] || (( $EPOCHSECONDS - $_nmk_update_ssh_socket_last_check > 300 )) && {
        eval $(tmux show-environment -s)
    }
    _nmk_update_ssh_socket_last_check=$EPOCHSECONDS
}

(( ${+commands[kubectl]} )) && {
    _nmk_precmd_functions+=_nmk-kubectl-precmd
    _nmk_preexec_functions+=_nmk-kubectl-preexec
}

[[ -n $TMUX && -n $SSH_CONNECTION && -S $SSH_AUTH_SOCK ]] && {
    _nmk_precmd_functions+=_nmk-update-ssh-socket
}

_nmk_precmd() {
    local hook
    for hook in $_nmk_precmd_functions; do
        $hook
    done
}

_nmk_preexec() {
    local hook
    for hook in $_nmk_preexec_functions; do
        $hook
    done
}

add-zsh-hook precmd  _nmk_precmd
add-zsh-hook preexec _nmk_preexec
# Detect & load version managers
() {
    typeset -a managers
    # Detect nvm
    [[ -e $HOME/.nvm/nvm.sh ]] && {
        managers+=(nvm)
        function init-nvm {
            local cmd
            cmd='source $HOME/.nvm/nvm.sh'
            # avoid calling `nvm use` again
            (( ${+NVM_BIN} )) && cmd+=' --no-use'
            eval "$cmd"
        }
    }
    # Detect pyenv
    (( ${+commands[pyenv]} )) && {
        managers+=(pyenv)
        function init-pyenv {
            integer has_virtualenv
            typeset -a pyenv_commands
            pyenv_commands=($(pyenv commands))
            [[ ${pyenv_commands[(r)virtualenv]} == virtualenv ]] \
                && ((has_virtualenv = 1))
            if (( ${+PYENV_SHELL} )); then
                eval "$(pyenv init - --no-rehash zsh)"
            else
                eval "$(pyenv init - zsh)"
            fi
            if (( has_virtualenv )); then
                # see https://github.com/pyenv/pyenv-virtualenv#activate-virtualenv
                # eval "$(pyenv virtualenv-init - zsh)"
                function virtualenv-init {
                    eval "$(pyenv virtualenv-init - zsh)"
                    unfunction virtualenv-init
                }
            fi
        }
    }
    # Detect rbenv
    (( ${+commands[rbenv]} )) && {
        managers+=(rbenv)
        function init-rbenv {
            if (( ${+RBENV_SHELL} )); then
                eval "$(rbenv init - --no-rehash zsh)"
            else
                eval "$(rbenv init - zsh)"
            fi
        }
    }
    # set default value if nmk_version_managers is unset
    (( ! ${+nmk_version_managers} )) && {
        typeset -ga nmk_version_managers
        nmk_version_managers=($managers)
    }
    local manager
    for manager in $nmk_version_managers; do
        case $manager in
            nvm ) init-nvm; unfunction init-nvm ;;
            pyenv ) init-pyenv; unfunction init-pyenv ;;
            rbenv ) init-rbenv; unfunction init-rbenv ;;
        esac
    done
}
[[ -e /etc/zsh_command_not_found ]] && source /etc/zsh_command_not_found
typeset -U path
() {
    local file
    for file ($ZDOTDIR/zshrc.extra.d/*.zsh(N)) source $file
}
source $ZDOTDIR/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh