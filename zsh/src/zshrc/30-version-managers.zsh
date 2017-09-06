# Detect & load version managers
() {
    typeset -a managers
    # Detect nvm
    [[ -e $HOME/.nvm/nvm.sh ]] && {
        managers+=(nvm)
        function init-nvm {
            source $HOME/.nvm/nvm.sh
        }
    }
    # Detect pyenv
    (( ${+commands[pyenv]} )) && {
        managers+=(pyenv)
        integer has_virtualenv
        integer has_virtualenvwrapper
        typeset -a pyenv_commands
        pyenv_commands=$(pyenv commands)
        [[ ${pyenv_commands[(r)virtualenv]} == virtualenv ]] \
            && ((has_virtualenv = 1))
        [[ ${pyenv_commands[(r)virtualenvwrapper]} == virtualenvwrapper ]] \
            && ((has_virtualenvwrapper = 1))
        function init-pyenv {
            eval "$(pyenv init -)"
            if (( has_virtualenv )); then
                eval "$(pyenv virtualenv-init -)"
            elif (( has_virtualenvwrapper )); then
                [[ $(pyenv version-name) != system* ]] && pyenv virtualenvwrapper
            fi
        }
    }
    # Detect rbenv
    (( ${+commands[rbenv]} )) && {
        managers+=(rbenv)
        function init-rbenv {
            eval "$(rbenv init -)"
        }
    }
    if [[ $NMK_AUTOLOAD != false ]]; then
        # set default value if nmk_version_managers is unset
        (( ! ${+nmk_version_managers} )) && {
            typeset -ga nmk_version_managers
            nmk_version_managers=($managers)
        }
        for manager in $nmk_version_managers; do
            case $manager in
                nvm ) init-nvm; unfunction init-nvm ;;
                pyenv ) init-pyenv; unfunction init-pyenv ;;
                rbenv ) init-rbenv; unfunction init-rbenv ;;
            esac
        done
    fi
}
