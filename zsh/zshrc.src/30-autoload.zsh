# Detect & load version managers
() {
    local nvm_hook_file="$HOME/.nvm/nvm.sh"
    typeset -a managers
    [[ -e $HOME/.nvm/nvm.sh ]] && {
        managers+=(nvm)
        function use-nvm() {
            source $HOME/.nvm/nvm.sh
        }
    }
    (( ${+commands[pyenv]} )) && {
        managers+=(pyenv)
        function use-pyenv() {
            eval "$(pyenv init -)"
            [[ ${$(pyenv commands)[(r)virtualenvwrapper]} == virtualenvwrapper ]] \
                && pyenv virtualenvwrapper
        }
    }
    (( ${+commands[rbenv]} )) && {
        managers+=(rbenv)
        function use-rbenv() {
            eval "$(rbenv init -)"
        }
    }
    if [[ $NMK_AUTOLOAD != false ]]; then
        # set default value if nmk_version_managers is unset
        if ! (($+nmk_version_managers)); then
            typeset -ga nmk_version_managers
            nmk_version_managers=($managers)
        fi
        for manager in $nmk_version_managers; do
            case $manager in
                nvm ) use-nvm; unfunction use-nvm ;;
                pyenv ) use-pyenv; unfunction use-pyenv ;;
                rbenv ) use-rbenv; unfunction use-rbenv ;;
            esac
        done
    fi
}
