() {
    local file
    for file ($ZDOTDIR/zshrc.extra.d/*.zsh(N)) source $file
}
