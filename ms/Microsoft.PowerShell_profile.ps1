New-Alias grrr git-reset-to-remote-branch.ps1

function _NMK_GIT_LOG_ONE_LINE { git log --oneline --decorate --graph --color=auto $args }
New-Alias lol _NMK_GIT_LOG_ONE_LINE

function _NMK_GIT_PULL_REBASE { git pull --rebase $args }
New-Alias gpr _NMK_GIT_PULL_REBASE
