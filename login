# This script will be sourced by login shell bash/zsh

try_login_shell() {
    if [[ -x "$1" ]]; then
        exec "$1" --motd --login
    fi
}

try_login_shell ~/.nmk/bin/nmk
try_login_shell ~/.nmk/nmk/target/debug/nmk

exec -l $SHELL

# vi: ft=sh
