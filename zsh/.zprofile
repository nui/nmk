# By default, tmux creates login shell for new window.
# If zprofile is already sourced. It should not be sourced again.

_nmk_setup_completion() {
    local completions_dir=$NMK_DIR/zsh/completion
    local zshrc_extra_dir=$NMK_DIR/zsh/zshrc.extra.d
    local gcloud_completion='/usr/share/google-cloud-sdk/completion.zsh.inc'
    (( ${+commands[kubectl]} )) && kubectl completion zsh > $zshrc_extra_dir/kubectl-completion.zsh
    (( ${+commands[rustup]} )) && rustup completions zsh > $completions_dir/_rustup
    [[ -e $gcloud_completion ]] && ln -sf $gcloud_completion $zshrc_extra_dir/gcloud-completion.zsh
}

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
