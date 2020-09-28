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

    if [[ -n $TMUX && -n $NMK_TMUX_VERSION ]]; then
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
