#!/bin/sh
set -x

VERSION=${1:-v$(date --iso)}

if ! [ -e bin/nmk -a -e bundle.sh ]; then
    >&2 echo "This script need to run from inside NMK directory"
    exit 1
fi

# don't run this on working environment
if [ -e zsh/.zcompdump -o -e zsh/.zsh_history ]; then
    >&2 echo "Aborted"
    exit 1
fi

# remove all .git directories
find . -type d -name .git -exec rm -rf {} +
echo $VERSION > VERSION
find . ! -type d -print0 | sort --reverse --zero-terminated > files.lst
# remove the write permission to prevent accidentally editing
<files.lst xargs --null chmod u-w
find . -mindepth 1 -type d -print0 | sort --reverse --zero-terminated > dirs.lst

cat > remove.sh << 'EOF'
#!/bin/sh
set -e
find . -name '*.pyc' -delete
find . -name __pycache__ -delete
<files.lst xargs --null rm
<dirs.lst xargs --null rmdir --ignore-fail-on-non-empty
exec rm dirs.lst remove.sh
EOF

# use fakeroot to prevent including user information in tar archive
fakeroot tar caf ../nmk.tar.gz --transform 's#^.#.nmk#' .
rm -f bundle.sh
