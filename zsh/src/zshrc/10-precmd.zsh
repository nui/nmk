_nmk-precmd-kubectl-hook() {
    if [[ -n $KUBECTL_CONTEXT ]]; then
        alias kubectl="kubectl --context=$KUBECTL_CONTEXT"
    elif (( ${+aliases[kubectl]} )); then
        unalias kubectl
    fi
}

_nmk_precmd_hooks=()
(( ${+commands[kubectl]} )) && _nmk_precmd_hooks+=_nmk-precmd-kubectl-hook

_nmk-precmd-hook() {
    local hook
    for hook in $_nmk_precmd_hooks; do
        $hook
    done
}
add-zsh-hook precmd _nmk-precmd-hook
