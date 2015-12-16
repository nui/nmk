# Hide user and host in prompt if NMK_DEVELOPMENT is true by default,
# this is not apply to zsh in ssh session
[[ $NMK_DEVELOPMENT == true && -z $SSH_TTY ]] \
    && horizontal_show_userhost=${horizontal_show_userhost:-0}

# Don't display git branch symbol if terminal does not support 256 colors
type -p tput &>/dev/null && (( $(command tput colors) < 256 )) && horizontal_git_branch_symbol=

prompt horizontal
