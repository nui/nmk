# Fix local installation of zsh via nmkpkg
# While compiling local zsh, PREFIX is set to /nmk-local.
# We have to change fpath at runtime to match installation directory
export FPATH=${FPATH:gs#/nmk-local#${NMK_HOME}/local#}

