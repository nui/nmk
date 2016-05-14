# This is the place to declare variables that control logic in zshrc.src/*.zsh
[[ -e $ZDOTDIR/zshrc.pre ]] && source $ZDOTDIR/zshrc.pre
autoload -Uz edit-command-line && zle -N edit-command-line
autoload -Uz promptinit && promptinit

setopt extendedglob
setopt histignorealldups
setopt histignorespace
setopt sharehistory

# force emacs keybindings
bindkey -e
# Search backwards and forwards with a pattern
bindkey '^R' history-incremental-pattern-search-backward
bindkey '^S' history-incremental-pattern-search-forward

bindkey '^X^E' edit-command-line

HISTFILE="${ZDOTDIR}/.zsh_history"
HISTSIZE=4000
SAVEHIST=$HISTSIZE
autoload -Uz compinit && compinit
zstyle ':completion:*' auto-description 'specify: %d'
zstyle ':completion:*' completer _expand _complete _correct _approximate
zstyle ':completion:*' format 'Completing %d'
zstyle ':completion:*' group-name ''
zstyle ':completion:*' menu select=2
type -p dircolors &>/dev/null && { 
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
zstyle ':completion:*:kill:*' command 'ps -u $USER -o pid,%cpu,cmd'
# see http://superuser.com/questions/378018/how-can-i-do-ctrl-z-and-bg-in-one-keypress-to-make-process-continue-in-backgroun
nmk-fancy-ctrl-z () {
    if [[ ${#BUFFER} -eq 0 ]]; then
        bg
        zle redisplay
    else
        zle push-input
    fi
}
zle -N nmk-fancy-ctrl-z
bindkey '^Z' nmk-fancy-ctrl-z
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
    if xclip -o &> /dev/null; then
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

# Disable terminal flow control, so that we can use '^S'
# for history-search-forward.
stty -ixon
# Hide user and host in prompt if NMK_DEVELOPMENT is true by default,
# this is not apply to zsh in ssh session
[[ $NMK_DEVELOPMENT == true && -z $SSH_TTY ]] \
    && horizontal_show_userhost=${horizontal_show_userhost:-0}

# Don't display git branch symbol if terminal does not support 256 colors
type -p tput &>/dev/null && (( $(command tput colors) < 256 )) && horizontal_git_branch_symbol=

prompt horizontal
# Autoload tools
() {
    local nvm_hook_file="$HOME/.nvm/nvm.sh"
    if [[ $NMK_AUTOLOAD != false ]]; then
        # set default value if nmk_load_tools is unset
        if ! (($+nmk_load_tools)); then
            typeset -ga nmk_load_tools
            [[ -e $nvm_hook_file ]] && nmk_load_tools+=(nvm)
            hash pyenv 2> /dev/null && nmk_load_tools+=(pyenv)
            hash rbenv 2> /dev/null && nmk_load_tools+=(rbenv)
        fi
        for tool in $nmk_load_tools; do
            case $tool in
                nvm )
                    source $nvm_hook_file ;;
                pyenv )
                    eval "$(pyenv init -)"
                    [[ ${$(pyenv commands)[(r)virtualenvwrapper]} == virtualenvwrapper ]] \
                        && pyenv virtualenvwrapper ;;
                rbenv )
                    eval "$(rbenv init -)" ;;
            esac
        done
    fi
}
[[ -e /etc/zsh_command_not_found ]] && source /etc/zsh_command_not_found
source $ZDOTDIR/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh
# auto change directory to somewhere if set
if [[ -n $NMK_RESPAWN_PANE_DIR ]]; then
    cd $NMK_RESPAWN_PANE_DIR
    unset NMK_RESPAWN_PANE_DIR
fi
typeset -U path
if [[ $NMK_IGNORE_LOCAL != true ]]; then
    [[ -e $ZDOTDIR/zshrc.extra ]] && source $ZDOTDIR/zshrc.extra
    for file ($ZDOTDIR/zshrc.extra.d/*.zsh(N)) {source $file}
    if [[ -e $NMK_ZSHRC_EXTRA ]]; then
        >&2 print -- "Zsh: load extra configuration from $NMK_ZSHRC_EXTRA"
        source $NMK_ZSHRC_EXTRA
    fi
fi
