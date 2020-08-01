# Change the current working directory to the directory of this script
cd $(dirname -- "$(readlink -f -- "$0")") # linux only
cd $(cd -P -- "$(dirname -- "$0")" && pwd -P) # doesn't work properly if $0 is a symbolic link
