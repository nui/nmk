autoload -Uz reset

() {
    local min_tmout=$(( 24*3600 ))
    # if TMOUT is set on some environment, extend it to 24 hours
    [[ $TMOUT = <-> ]] && (( $TMOUT <= $min_tmout )) && export TMOUT=$(( $min_tmout ))
}

