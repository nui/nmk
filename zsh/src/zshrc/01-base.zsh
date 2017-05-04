autoload -Uz edit-command-line && zle -N edit-command-line
autoload -Uz promptinit && promptinit

setopt extendedglob
setopt histignorealldups
setopt histignorespace
setopt sharehistory
setopt dvorak

# force emacs keybindings
bindkey -e
# Search backwards and forwards with a pattern
bindkey '^R' history-incremental-pattern-search-backward
bindkey '^S' history-incremental-pattern-search-forward

bindkey '^X^E' edit-command-line

# Fix Home, End, and Delete Key in build-from-source tmux
bindkey ${terminfo[khome]} beginning-of-line
bindkey ${terminfo[kend]}  end-of-line
bindkey ${terminfo[kdch1]} delete-char

HISTFILE="${ZDOTDIR}/.zsh_history"
HISTSIZE=2500
SAVEHIST=$HISTSIZE
