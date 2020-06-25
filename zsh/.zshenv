if [[ $NMK_ZSH_GLOBAL_RCS == "0" ]]; then
    unsetopt GLOBAL_RCS
fi

(( ! ${+NMK_HOME} )) && export NMK_HOME=${ZDOTDIR:h}
(( ! ${+VIMINIT} )) && export VIMINIT='source $NMK_HOME/vim/init.vim'

fpath=(
    $ZDOTDIR/functions
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
