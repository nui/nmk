#!/bin/sh
export SSH_AUTH_SOCK=$HOME/.gnupg/S.gpg-agent.ssh
# Change stack size (should only apply to macos?)
# ulimit -a
# ulimit -s 32768
exec "/Applications/JetBrains Toolbox.app/Contents/MacOS/jetbrains-toolbox"
