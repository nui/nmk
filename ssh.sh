# POSIX compatible script

check_exec() {
    if [ -e "$1" ]; then
        exec "$1" --ssh --login
    fi
}

check_exec "$HOME/.nmk/bin/nmk"

for build_type in debug release; do
    check_exec "$HOME/.nmk/nmk.rs/target/$build_type/nmk"
done

exec $SHELL -l
