# POSIX compatible script

try_exec_login_shell() {
    if [ -e "$1" ]; then
        exec "$1" --ssh --login
    fi
}

if [ -e "$HOME" ]; then
    try_exec_login_shell "$HOME/.nmk/bin/nmk"
    try_exec_login_shell "$HOME/bin/nmk"
fi
try_exec_login_shell "/usr/local/bin/nmk"

# This may not work on dash, but who use dash as a login shell?
exec -l $SHELL
