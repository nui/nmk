#!/usr/bin/env zsh

(( ! ${+NMK_DIR} )) && {
    print -- '$NMK_DIR is unset'
    exit 1
}

setup_completion() {
    local completions_dir=$ZDOTDIR/completion
    (( ${+commands[kubectl]} )) && kubectl completion zsh > $completions_dir/_kubectl
    (( ${+commands[rustup]} ))  && rustup completions zsh > $completions_dir/_rustup
}

precompile_nvm() {
    [[ -d $NVM_DIR ]] && zcompile $NVM_DIR/nvm.sh
}

setup_completion
precompile_nvm

