#!/bin/sh
set -x

VERSION=${1:-$(date --iso)}

if ! [ -e bin/nmk -a -e bundle.sh ]; then
    >&2 echo "This script need to run from inside NMK directory"
    exit 1
fi

# don't run this on working environment
if [ -e zsh/.zcompdump -o -e zsh/.zsh_history -o -d node_modules ]; then
    >&2 echo "Aborted"
    exit 1
fi

# remove this script
rm -f bundle.sh
# remove all .git directories
find . -type d -name .git -exec rm -rf {} +
echo $VERSION > VERSION
find . ! -type d -print0 | sort --reverse --zero-terminated > .bundle-files
# remove the write permission to prevent accidentally editing
<.bundle-files xargs --null chmod u-w
find . -mindepth 1 -type d -print0 | sort --reverse --zero-terminated > .bundle-dirs

cat > remove.sh << 'EOF'
#!/bin/sh
set -e
find . -name __pycache__ -exec rm -rf {} +
find . -name '*.pyc' -delete
<.bundle-files xargs --null rm
<.bundle-dirs xargs --null rmdir --ignore-fail-on-non-empty
exec rm .bundle-dirs remove.sh
EOF

# use fakeroot to prevent including user information in tar archive
fakeroot tar caf ../nmk.tar.gz --transform 's#^.#.nmk#' .

