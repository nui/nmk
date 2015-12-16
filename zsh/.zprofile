# By default, windows under tmux are login shell, and they will source .zprofile
# which is already sourced by the login shell. To prevent login shell under
# login shell source this file again, check for some environment variable
# and do nothing if variable is exist.
if [[ $NMK_ZPROFILE_SOURCED != true && -e $ZDOTDIR/zprofile ]]; then
    export NMK_ZPROFILE_SOURCED=true
    source $ZDOTDIR/zprofile
fi
# vi: ft=zsh
