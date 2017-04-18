[[ -e $ZDOTDIR/zshrc.extra ]] && source $ZDOTDIR/zshrc.extra
for file ($ZDOTDIR/zshrc.extra.d/*.zsh(N)) {source $file}
