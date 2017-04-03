if [[ $NMK_IGNORE_LOCAL != true ]]; then
    [[ -e $ZDOTDIR/zshrc.extra ]] && source $ZDOTDIR/zshrc.extra
    for file ($ZDOTDIR/zshrc.extra.d/*.zsh(N)) {source $file}
    if [[ -e $NMK_ZSHRC_EXTRA ]]; then
        >&2 print -- "Zsh: load extra configuration from $NMK_ZSHRC_EXTRA"
        source $NMK_ZSHRC_EXTRA
    fi
fi
