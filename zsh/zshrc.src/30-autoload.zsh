# Autoload tools
() {
    local nvm_hook_file="$HOME/.nvm/nvm.sh"
    if [[ $NMK_AUTOLOAD != false ]]; then
        # set default value if nmk_load_tools is unset
        if ! (($+nmk_load_tools)); then
            typeset -ga nmk_load_tools
            [[ -e $nvm_hook_file ]] && nmk_load_tools+=(nvm)
            hash pyenv 2> /dev/null && nmk_load_tools+=(pyenv)
            hash rbenv 2> /dev/null && nmk_load_tools+=(rbenv)
        fi
        for tool in $nmk_load_tools; do
            case $tool in
                nvm )
                    source $nvm_hook_file ;;
                pyenv )
                    eval "$(pyenv init -)"
                    [[ ${$(pyenv commands)[(r)virtualenvwrapper]} == virtualenvwrapper ]] \
                        && pyenv virtualenvwrapper ;;
                rbenv )
                    eval "$(rbenv init -)" ;;
            esac
        done
    fi
}
