# Hack for arch linux
# Do not load zsh global configuration files in arch linux
# In arch linux, /etc/zsh/zprofile contains a line that will source everything
# from /etc/profile. And they do reset $PATH completely.
# It makes PATH set by nmk unusable
[[ -f /etc/arch-release ]] && setopt noglobalrcs

fpath=(
    # My completion
    $ZDOTDIR/completion
    # My theme
    $ZDOTDIR/themes
    # Plugin completion
    $ZDOTDIR/plugins/zsh-completions/src

    $fpath
)

() {
    local -a local_completion
    if [[ $NMK_IGNORE_LOCAL != true ]]; then
        local_completion=($ZDOTDIR/local/completion/_*(N))
        if ((${#local_completion} > 0)); then
            # prepend to $fpath
            fpath[1,0]=$ZDOTDIR/local/completion
        fi
    fi
}

if [[ $NMK_IGNORE_LOCAL != true && -e $ZDOTDIR/zshenv.extra ]]; then
    source $ZDOTDIR/zshenv.extra
fi
