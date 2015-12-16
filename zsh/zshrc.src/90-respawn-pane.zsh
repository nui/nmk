# auto change directory to somewhere if set
if [[ -n $NMK_RESPAWN_PANE_DIR ]]; then
    cd $NMK_RESPAWN_PANE_DIR
    unset NMK_RESPAWN_PANE_DIR
fi
