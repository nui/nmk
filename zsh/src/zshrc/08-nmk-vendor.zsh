# Fix vendored zsh. We have to change fpath at runtime to match installation directory.
() {
    # prefix must end with : to make below hack work for all possible input
    local prefix="/prefix:"
    FPATH=$prefix$FPATH
    # this is a hacky way to fix path begin with /nmk-vendor
    FPATH=${FPATH:gs#:/nmk-vendor#:${NMK_HOME}/vendor#}
    FPATH=${FPATH#"$prefix"}
}

