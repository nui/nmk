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

    # Try to cache tmux version to reduce nmk startup time.
    #
    # This code is meant to run on real login shell such as
    #   - Linux remote login to server
    #   - Linux local login via display manager
    # And not run on following shell/terminal
    #   - OSX (every shell is login shell)
    #   - JetBrains terminal
    #   - Tmux (already guard by outer if block)
    if [[ -z $TERMINAL_EMULATOR ]] && [[ -z $TERM_PROGRAM ]] && ((${+commands[tmux]})); then
        export NMK_TMUX_VERSION="$(tmux -V | cut -d ' ' -f 2)"
    fi
fi
# vi: ft=zsh
