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
    typeset -a additional_fpath
    additional_fpath=(
        /usr/share/zsh/vendor-functions
        /usr/share/zsh/vendor-completions
    )
    for dir in $additional_fpath; do
        if [[ -d $dir && ${fpath[(r)$dir]} != $dir ]]; then
            fpath+=$dir
        fi
    done
}


if [[ -e $ZDOTDIR/zshenv.extra ]]; then
    source $ZDOTDIR/zshenv.extra
fi

