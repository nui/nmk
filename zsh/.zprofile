# By default, tmux creates login shell for new window.
# If zprofile is already sourced. It should not be sourced again.

_nmk_post_login() {
    local completions_dir=$NMK_DIR/zsh/completion
    local zshrc_extra_dir=$NMK_DIR/zsh/zshrc.extra.d
    local gcloud_completion='/usr/share/google-cloud-sdk/completion.zsh.inc'
    (( ${+commands[kubectl]} )) && kubectl completion zsh > $zshrc_extra_dir/kubectl-completion.zsh
    (( ${+commands[rustup]} )) && rustup completions zsh > $completions_dir/_rustup
    [[ -e $gcloud_completion ]] && ln -sf $gcloud_completion $zshrc_extra_dir/gcloud-completion.zsh
}

# NMK_ZPROFILE_SOURCED is set and check to prevent above situation.
if [[ $NMK_ZPROFILE_SOURCED != true && -e $ZDOTDIR/zprofile ]]; then
    source $ZDOTDIR/zprofile
    export NMK_ZPROFILE_SOURCED=true

    _nmk_post_login
fi

unfunction _nmk_post_login
# vi: ft=zsh
