#!/bin/sh
set -x

if ! [ -e bin/nmk -a -e bundle.sh ]; then
    >&2 echo "This script need to run from NMK root directory"
    exit 1
fi

# don't run this on working environment
if [ -e zsh/.zcompdump -o -e zsh/.zsh_history -o -d node_modules ]; then
    >&2 echo "Aborted"
    exit 1
fi

ORIG_PWD=$PWD

date --rfc-3339=seconds > BUILD_TIME
# add uninstall script
cat > uninstall.sh << 'EOF'
#!/bin/sh
set -e
find . -name '*.pyc' -exec rm -f {} +
<.bundle-files xargs --null rm
EOF
rm -f bundle.sh

OUT_TAR=$ORIG_PWD/nmk.tar.gz
TMP_DIR=$(mktemp -d)

tar -c --exclude-vcs --exclude-from=.nmkignore . | tar -x -C "$TMP_DIR"
cd $TMP_DIR
# create a list of files
find . ! -type d -print0 | sort --reverse --zero-terminated > .bundle-files
# unset write permission to get warning message on update read-only files
find . -type f -exec chmod ugo-w {} +
tar -caf "$OUT_TAR" --owner=0 --group=0 --mtime='' --transform 's#^\./#.nmk/#' .

rm -rf $TMP_DIR

