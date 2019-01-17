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

