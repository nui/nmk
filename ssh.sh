# POSIX compatible script

try_zsh_login() {
    if [ -x "$1" -a -f "$1" ]; then
        exec "$1" --motd --login
    fi
}

if [ -d "$HOME" ]; then
    try_zsh_login "$HOME/.nmk/bin/nmk"
    try_zsh_login "$HOME/bin/nmk"
fi
try_zsh_login "/usr/local/bin/nmk"

# This may not work on dash, but who use dash as a login shell?
exec -l $SHELL
