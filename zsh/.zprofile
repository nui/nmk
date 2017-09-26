# By default, tmux creates login shell for new window.
# If zprofile is already sourced. It should not be sourced again.

# NMK_ZPROFILE_SOURCED is set and check to prevent above situation.
if [[ $NMK_ZPROFILE_SOURCED != true && -e $ZDOTDIR/zprofile ]]; then
    source $ZDOTDIR/zprofile
    export NMK_ZPROFILE_SOURCED=true
fi
# vi: ft=zsh
