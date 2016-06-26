# Detect & load version managers
() {
    typeset -a managers
    # Detect nvm
    [[ -e $HOME/.nvm/nvm.sh ]] && {
        managers+=(nvm)
        function use-nvm() {
            source $HOME/.nvm/nvm.sh
        }
    }
    # Detect pyenv
    (( ${+commands[pyenv]} )) && {
        managers+=(pyenv)
        function use-pyenv() {
            eval "$(pyenv init -)"
            [[ ${$(pyenv commands)[(r)virtualenvwrapper]} == virtualenvwrapper ]] \
                && pyenv virtualenvwrapper
        }
    }
    # Detect rbenv
    (( ${+commands[rbenv]} )) && {
        managers+=(rbenv)
        function use-rbenv() {
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
                nvm ) use-nvm; unfunction use-nvm ;;
                pyenv ) use-pyenv; unfunction use-pyenv ;;
                rbenv ) use-rbenv; unfunction use-rbenv ;;
            esac
        done
    fi
}
