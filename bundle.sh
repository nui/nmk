#!/bin/sh
set -x

if ! [ -e bin/nmk -a -e bundle.sh ]; then
    >&2 echo "This script need to run from inside NMK directory"
    exit 1
fi

# don't run this on working environment
if [ -e zsh/.zcompdump -o -e zsh/.zsh_history -o -d node_modules ]; then
    >&2 echo "Aborted"
    exit 1
fi

rm -f bundle.sh
git --no-pager log -3 --oneline --color=never > LAST_COMMITS
date --rfc-3339=seconds > BUILD_TIME
# add uninstall script
cat > uninstall.sh << 'EOF'
#!/bin/sh
set -e
find . -name '*.pyc' -exec rm -f {} +
<.bundle-files xargs --null rm
<.bundle-dirs xargs --null rmdir --ignore-fail-on-non-empty
exec rm -f .bundle-dirs
EOF
# remove all .git directories
find . -type d -name .git -exec rm -rf {} +

find . ! -type d -print0 | sort --reverse --zero-terminated > .bundle-files
find . -mindepth 1 -type d -print0 | sort --reverse --zero-terminated > .bundle-dirs

# unset write permission to get warning message on update file
find . -type f -exec chmod ugo-w {} +

tar caf ../nmk.tar.xz --owner=0 --group=0 --mtime='' --transform 's#^\./#.nmk/#' .

