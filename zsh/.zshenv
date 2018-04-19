# Hack for arch linux
# Do not load zsh global configuration files in arch linux
# In arch linux, /etc/zsh/zprofile contains a line that will source everything
# from /etc/profile. And they do reset $PATH completely.
# It makes PATH set by nmk unusable
[[ -f /etc/arch-release ]] && unsetopt GLOBAL_RCS

fpath=(
    $ZDOTDIR/fpath
    # My completion
    $ZDOTDIR/completion
    # My theme
    $ZDOTDIR/themes
    # Plugin completion
    $ZDOTDIR/plugins/zsh-completions/src

    $fpath
)

if [[ -e $ZDOTDIR/zshenv.extra ]]; then
    source $ZDOTDIR/zshenv.extra
fi
