New-Alias grrr git-reset-to-remote-branch.ps1

function _NMK_GIT_LOG_ONE_LINE { git log --oneline --decorate --graph --color=auto $args }
New-Alias lol _NMK_GIT_LOG_ONE_LINE

function _NMK_GIT_PULL_REBASE { git pull --rebase $args }
New-Alias gpr _NMK_GIT_PULL_REBASE

function _GIT_CHECKOUT { git checkout $args }
New-Alias gco _GIT_CHECKOUT

function _GIT_DIFF { git diff $args }
New-Alias gd _GIT_DIFF

function _GIT_DIFF_STAGED { git diff --staged $args }
New-Alias gds _GIT_DIFF_STAGED

function _GIT_RESET_HARD { git reset --hard $args }
New-Alias grh _GIT_RESET_HARD

function _GIT_STATUS { git status $args }
New-Alias gs _GIT_STATUS

