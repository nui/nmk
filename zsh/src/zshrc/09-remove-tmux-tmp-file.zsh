# Remove temporary tmux configuration file
if [[ -n $TMUX && -n $NMK_TMUX_TEMP_CONF ]]; then
    tmux set-environment -gr NMK_TMUX_TEMP_CONF
    rm $NMK_TMUX_TEMP_CONF
    unset NMK_TMUX_TEMP_CONF
fi
