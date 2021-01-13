if [[ $NMK_ZSH_GLOBAL_RCS == "0" ]]; then
    unsetopt GLOBAL_RCS
fi

() {
    setopt localoptions histsubstpattern
    fpath=(
        $ZDOTDIR/functions
        $ZDOTDIR/fpath
        # My completion
        $ZDOTDIR/completion
        # My theme
        $ZDOTDIR/themes
        # Plugin completion
        $ZDOTDIR/plugins/zsh-completions/src

        # Fix hard-coded path of vendored zsh.
        # When we compile zsh, installation path is set to /nmk-vendor.
        # We have to change fpath at runtime to match actual installation directory.
        ${fpath:s|#/nmk-vendor|${NMK_HOME}/vendor|}
    )
}


if [[ -e $ZDOTDIR/zshenv.extra ]]; then
    source $ZDOTDIR/zshenv.extra
fi

