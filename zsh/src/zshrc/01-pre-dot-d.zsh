() {
    local file
    for file ($ZDOTDIR/zshrc.pre.d/*.zsh(N)) source $file
}
