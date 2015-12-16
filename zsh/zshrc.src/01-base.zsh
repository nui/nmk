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
