_nmk_setup_completion() {
    local completions_dir=$ZDOTDIR/completion
    local zshrc_extra_dir=$ZDOTDIR/zshrc.extra.d
    (( ${+commands[kubectl]} )) && kubectl completion zsh > $completions_dir/_kubectl
    (( ${+commands[rustup]} ))  && rustup completions zsh > $completions_dir/_rustup
}

# By default, tmux creates login shell for new window.
# If zprofile is already sourced. It should not be sourced again.
# NMK_PROFILE_INITIATED is set and check to prevent above situation.
if [[ $NMK_PROFILE_INITIATED != true ]]; then
    if [[ -e $ZDOTDIR/zprofile ]]; then
        source $ZDOTDIR/zprofile
    fi
    _nmk_setup_completion
    export NMK_PROFILE_INITIATED=true
fi

unfunction _nmk_setup_completion
# vi: ft=zsh
