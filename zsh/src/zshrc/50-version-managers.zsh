# Detect & load version managers
() {
    typeset -a managers
    # Detect nvm
    [[ -e $HOME/.nvm/nvm.sh ]] && {
        managers+=(nvm)
        function init-nvm {
            local cmd
            cmd='source $HOME/.nvm/nvm.sh'
            # avoid calling `nvm use` again
            (( ${+NVM_BIN} )) && cmd+=' --no-use'
            eval "$cmd"
        }
    }
    # Detect pyenv
    (( ${+commands[pyenv]} )) && {
        managers+=(pyenv)
        function init-pyenv {
            integer has_virtualenv
            typeset -a pyenv_commands
            pyenv_commands=($(pyenv commands))
            [[ ${pyenv_commands[(r)virtualenv]} == virtualenv ]] \
                && ((has_virtualenv = 1))
            if (( ${+PYENV_SHELL} )); then
                eval "$(pyenv init - --no-rehash zsh)"
            else
                eval "$(pyenv init - zsh)"
            fi
            if (( has_virtualenv )); then
                # see https://github.com/pyenv/pyenv-virtualenv#activate-virtualenv
                # eval "$(pyenv virtualenv-init - zsh)"
                function virtualenv-init {
                    eval "$(pyenv virtualenv-init - zsh)"
                    unfunction virtualenv-init
                }
            fi
        }
    }
    # Detect rbenv
    (( ${+commands[rbenv]} )) && {
        managers+=(rbenv)
        function init-rbenv {
            if (( ${+RBENV_SHELL} )); then
                eval "$(rbenv init - --no-rehash zsh)"
            else
                eval "$(rbenv init - zsh)"
            fi
        }
    }
    # set default value if nmk_version_managers is unset
    (( ! ${+nmk_version_managers} )) && {
        typeset -ga nmk_version_managers
        nmk_version_managers=($managers)
    }
    local manager
    for manager in $nmk_version_managers; do
        case $manager in
            nvm ) init-nvm; unfunction init-nvm ;;
            pyenv ) init-pyenv; unfunction init-pyenv ;;
            rbenv ) init-rbenv; unfunction init-rbenv ;;
        esac
    done
}
