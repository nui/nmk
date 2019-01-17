() {
    local file
    for file ($ZDOTDIR/zshrc.pre.d/*.zsh(N)) {
        source $file
    }
}
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

HISTFILE="${ZDOTDIR}/.zsh_history"
HISTSIZE=2500
SAVEHIST=$HISTSIZE
autoload -Uz compinit && compinit
zstyle ':completion:*' auto-description 'specify: %d'
zstyle ':completion:*' completer _expand _complete _correct _approximate
zstyle ':completion:*' format 'Completing %d'
zstyle ':completion:*' group-name ''
zstyle ':completion:*' menu select=2
(( ${+commands[dircolors]} )) && {
    eval "$(dircolors -b)"
    zstyle ':completion:*:default' list-colors ${(s.:.)LS_COLORS}
}
zstyle ':completion:*' list-colors ''
zstyle ':completion:*' list-prompt %SAt %p: Hit TAB for more, or the character to insert%s
zstyle ':completion:*' matcher-list '' 'm:{a-z}={A-Z}' 'm:{a-zA-Z}={A-Za-z}' 'r:|[._-]=* r:|=* l:|=*'
zstyle ':completion:*' menu select=long
zstyle ':completion:*' select-prompt %SScrolling active: current selection at %p%s
zstyle ':completion:*' verbose true
zstyle ':completion:*:*:kill:*:processes' list-colors '=(#b) #([0-9]#)*=0=01;31'
zstyle ':completion:*:kill:*' command 'ps -a -o tty,pid,%cpu,cmd k %cpu'
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

rf() {
    local -a list
    local _path
    # relative path
    if (( ${+2} )); then
        _path=$(realpath --relative-to=$2 -- $1)
    # absolute path
    else
        _path=${1:a}
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
    # pipe to pbcopy if present
    elif (( ${+commands[pbcopy]} )); then
        list+='> >(pbcopy)'
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

# vi = Vim without plugins
(( ${+commands[vi]} )) && {
    alias vi='env -u VIMINIT vi'
}

[[ -n $EDITOR ]] && alias neo=$EDITOR

# apply tmux session environment to running shell
alias ssenv=' eval $(tmux show-environment -s)'

# see http://superuser.com/questions/378018/how-can-i-do-ctrl-z-and-bg-in-one-keypress-to-make-process-continue-in-backgroun
_nmk-fancy-ctrl-z() {
    if [[ ${#BUFFER} -eq 0 ]]; then
        bg
        zle redisplay
    else
        zle push-input
    fi
}
zle -N _nmk-fancy-ctrl-z
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
        bindkey -r ${key[CtrlL]}
        _nmk-tmux-clear-history() {
            tput reset
            zle clear-screen
            tmux clear-history
        }
        zle -N _nmk-tmux-clear-history
        bind2maps emacs         -- CtrlL      _nmk-tmux-clear-history
    else
        bind2maps emacs         -- PageUp     redisplay
    fi
    # PageDown do nothing
    bind2maps emacs             -- PageDown   redisplay
    # Search backwards and forwards with a pattern
    bind2maps emacs -- CtrlR history-incremental-pattern-search-backward
    bind2maps emacs -- CtrlS history-incremental-pattern-search-forward

    bindkey '^X^E' edit-command-line
    bind2maps emacs -- CtrlZ _nmk-fancy-ctrl-z

    # Fix Home, End, and Delete Key in build-from-source tmux
    bind2maps emacs -- Home     beginning-of-line
    bind2maps emacs -- End      end-of-line
    bind2maps emacs -- Delete   delete-char

    unfunction bind2maps
}
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

_nmk_precmd_functions=()
_nmk_preexec_functions=()
(( ${+commands[kubectl]} )) && {
    _nmk_precmd_functions+=_nmk-kubectl-precmd
    _nmk_preexec_functions+=_nmk-kubectl-preexec
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
            source $HOME/.nvm/nvm.sh
        }
    }
    # Detect pyenv
    (( ${+commands[pyenv]} )) && {
        managers+=(pyenv)
        integer has_virtualenv
        typeset -a pyenv_commands
        pyenv_commands=$(pyenv commands)
        [[ ${pyenv_commands[(r)virtualenv]} == virtualenv ]] \
            && ((has_virtualenv = 1))
        function init-pyenv {
            eval "$(pyenv init -)"
            if (( has_virtualenv )); then
                eval "$(pyenv virtualenv-init -)"
            fi
        }
    }
    # Detect rbenv
    (( ${+commands[rbenv]} )) && {
        managers+=(rbenv)
        function init-rbenv {
            eval "$(rbenv init -)"
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
# zsh function implementation of main entrypoint
nmk() {
    local python=python
    if [[ -n $NMK_PYTHON ]]; then
        if [[ ! -x $NMK_PYTHON ]]; then
            >&2 print -- "$NMK_PYTHON not found"
            >&2 print -- 'Please update $NMK_PYTHON'
            return 1
        fi
        python=$NMK_PYTHON
    fi
    $python $NMK_DIR/bin/nmk.py "$@"
}
typeset -U path
() {
    local file
    for file ($ZDOTDIR/zshrc.extra.d/*.zsh(N)) {
        source $file
    }
}
source $ZDOTDIR/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh