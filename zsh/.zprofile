# By default, tmux creates login shell for new window.
# If zprofile is already sourced. It should not be sourced again.
# NMK_PROFILE_INITIATED is set and check to prevent above situation.
if [[ $NMK_PROFILE_INITIATED != true ]]; then
    (( ! ${+NMK_HOME} )) && export NMK_HOME=${ZDOTDIR:h}
    (( ! ${+VIMINIT} )) && export VIMINIT='source $NMK_HOME/vim/init.vim'

    if [[ -e $ZDOTDIR/zprofile ]]; then
        source $ZDOTDIR/zprofile
    fi
    export NMK_PROFILE_INITIATED=true
fi
# vi: ft=zsh
