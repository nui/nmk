# zsh function implementation of main entrypoint
nmk() {
    local python=python
    if [[ -n $NMK_PYTHON ]]; then
        if [[ ! -x $NMK_PYTHON ]]; then
            >&2 print -- "$NMK_PYTHON not found"
            >&2 print -- 'Please update $NMK_PYTHON'
            return 1
        fi
        python=$NMK_PYTHON
    fi
    $python $NMK_DIR/bin/nmk.py "$@"
}
