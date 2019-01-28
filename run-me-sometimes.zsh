#!/usr/bin/env zsh

setup_completion() {
    local completions_dir=$ZDOTDIR/completion
    (( ${+commands[kubectl]} )) && kubectl completion zsh > $completions_dir/_kubectl
    (( ${+commands[rustup]} ))  && rustup completions zsh > $completions_dir/_rustup
}

precompile_nvm() {
    local script=$HOME/.nvm/nvm.sh
    [[ -e $script ]] && zcompile $script
}

setup_completion
precompile_nvm

