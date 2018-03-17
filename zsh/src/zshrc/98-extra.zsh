[[ -e $ZDOTDIR/zshrc.extra ]] && source $ZDOTDIR/zshrc.extra
() {
    local file
    for file ($ZDOTDIR/zshrc.extra.d/*.zsh(N)) {
        source $file
    }
}
