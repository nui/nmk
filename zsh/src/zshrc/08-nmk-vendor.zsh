# Fix vendored zsh. We have to change fpath at runtime to match installation directory.
() {
    local next_path
    local prefix="/prefix:"
    next_path=$prefix$FPATH
    next_path=${next_path:gs#:/nmk-vendor#:${NMK_HOME}/vendor#}
    FPATH=${next_path#"$prefix"}
}

