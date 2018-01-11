_nmk-precmd-kubectl-hook() {
    if (( ${+kubectl_context} )); then
        alias kubectl="kubectl --context=$kubectl_context"
    elif (( ${+aliases[kubectl]} )); then
        unalias kubectl
    fi
}

_nmk_precmd_hooks=()
(( ${+commands[kubectl]} )) && _nmk_precmd_hooks+=_nmk-precmd-kubectl-hook

_nmk-precmd-hook() {
    for hook in $_nmk_precmd_hooks; do
        $hook
    done
}
add-zsh-hook precmd _nmk-precmd-hook
