# see http://superuser.com/questions/378018/how-can-i-do-ctrl-z-and-bg-in-one-keypress-to-make-process-continue-in-backgroun
function nmk-fancy-ctrl-z {
    if [[ ${#BUFFER} -eq 0 ]]; then
        bg
        zle redisplay
    else
        zle push-input
    fi
}
zle -N nmk-fancy-ctrl-z
bindkey '^Z' nmk-fancy-ctrl-z
