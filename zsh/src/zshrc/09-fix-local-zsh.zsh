# Fix local installation of zsh.
# We have to change fpath at runtime to match installation directory.
export FPATH=${FPATH:gs#/NmK-LoCaL_MaRkEr#${NMK_HOME}/local#}

