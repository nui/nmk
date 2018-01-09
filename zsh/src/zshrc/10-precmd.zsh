nmk-precmd-hook() {
    if (( ${+commands[kubectl]} )); then
        if (( ${+K8S_CONTEXT} )); then
            alias kubectl='kubectl --context=$K8S_CONTEXT'
        else
            alias kubectl=kubectl
        fi
    fi
}
add-zsh-hook precmd nmk-precmd-hook
