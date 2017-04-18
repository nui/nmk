# Detect & load version managers
function {
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
        function init-pyenv {
            eval "$(pyenv init -)"
            # Initialise virtualenvwrapper, skip if using system version
            [[ $(pyenv version-name) != system* ]] \
                && [[ ${$(pyenv commands)[(r)virtualenvwrapper]} == virtualenvwrapper ]] \
                && pyenv virtualenvwrapper
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
