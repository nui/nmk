# Hack for arch linux & alpine linux
# Do not load zsh global configuration files
# global zprofile contains a line that will source everything
# from /etc/profile. And they do reset $PATH completely.
# It makes PATH set by nmk unusable
[[ -f /etc/arch-release || -f /etc/alpine-release ]] && unsetopt GLOBAL_RCS

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
