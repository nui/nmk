# Remove temporary tmux configuration file
if [[ -n $TMUX && -e $NMK_TMUX_CONF ]]; then
    rm $NMK_TMUX_CONF
    unset NMK_TMUX_CONF
fi
