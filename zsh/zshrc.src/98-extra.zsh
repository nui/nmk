if [[ $NMK_IGNORE_LOCAL != true ]]; then
    [[ -e $ZDOTDIR/zshrc.extra ]] && source $ZDOTDIR/zshrc.extra
    for file ($ZDOTDIR/zshrc.extra.d/*.zsh(N)) {source $file}
fi
