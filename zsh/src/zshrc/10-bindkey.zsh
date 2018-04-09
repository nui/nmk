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

    unfunction bind2maps
}
