# Remove temporary tmux configuration file
if [[ -n $TMUX && -e $NMK_TMP_TMUX_CONF ]]; then
    rm $NMK_TMP_TMUX_CONF
    unset NMK_TMP_TMUX_CONF
fi
