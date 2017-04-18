if [[ $NMK_IGNORE_LOCAL != true && -e $ZDOTDIR/zshrc.pre ]]; then
    source $ZDOTDIR/zshrc.pre
fi
