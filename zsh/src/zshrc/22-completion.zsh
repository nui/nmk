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

    # Try to add completion directories to fpath
    local -a completion_dir
    completion_dir=(
        /usr/share/zsh/vendor-completions
    )
    for fp in $completion_dir; do
        # if $fp not in $fpath and $fp does exists
        if [[ ${fpath[(ie)$fp]} -gt ${#fpath} ]] && [[ -d $fp ]]; then
            fpath+=$fp
        fi
    done
}
