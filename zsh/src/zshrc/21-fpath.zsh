() {
    # Try to add directory to fpath
    local -a additional_fpath
    additional_fpath=(
        /usr/share/zsh/vendor-completions
    )
    for fp in $additional_fpath; do
        # if $fp not in $fpath and $fp does exists
        if [[ ${fpath[(ie)$fp]} -gt ${#fpath} ]] && [[ -d $fp ]]; then
            fpath+=$fp
        fi
    done
}
