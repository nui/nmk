# nmk
This repository contains my dot files and some scripts that I use everyday.


## System dependencies
```sh
# Debian system dependencies
sudo apt install git python tmux zsh

# Recommended packages
# For desktop
sudo apt install vim-gtk xclip
# For server
sudo apt install vim-nox
```


## Installation
```sh
# Github checkout
    git clone --recursive https://github.com/nuimk/nmk.git ~/.nmk
    ~/.nmk/bin/nmk
    ~/.nmk/vim/update-plugins
# or just grab the latest build and extract it (without git)
    curl -sSL https://storage.googleapis.com/nmk.nuimk.com/nmk.tar.gz | tar -C ~ -xzf -
    or
    curl -sSL nmk.nuimk.com/nmk.tar.gz | tar -C ~ -xzf -

# optional, install entrypoint
    curl -sSL https://storage.googleapis.com/nmk.nuimk.com/nmk.rs/nmk-amd64-linux-musl.gz | gunzip > ~/.nmk/bin/nmk
    chmod +x ~/.nmk/bin/nmk
```

## Binary entrypoint

If python is not install on target system, use following binary to start tmux

- [Linux](https://storage.googleapis.com/nmk.nuimk.com/nmk.rs/nmk-amd64-linux-musl.gz)
- [Mac](https://storage.googleapis.com/nmk.nuimk.com/nmk.go/nmk.go-amd64-darwin)
- [FreeBSD](https://storage.googleapis.com/nmk.nuimk.com/nmk.go/nmk.go-amd64-freebsd)


## Tmux navigation
```
F1 -> Next pane
F2 -> Last window
F3 -> Previous window
F4 -> Next window
F5 -> Zoom pane
F6 -> Choose tree

Shift+Fx  -> Function key
F12 Fx    -> Function key

F12 F12   -> detach
F12 (1-9) -> Select window number x
```


## Terminal setup
To use 256 colors, Set `TERM` environment variable to `xterm-256color`.

**Konsole** (KDE Terminal):
- Right click and choose `Edit Current Profiles`
- Click edit button next to environment label
- Change `TERM=xterm` to `TERM=xterm-256color`


## zsh outside tmux
Overwrite `~/.zshenv` with
```sh
export ZDOTDIR=~/.nmk/zsh
source $ZDOTDIR/.zshenv
```

Then run `cp ~/.nmk/zsh/{template/,}zprofile`

Log out and log back in.


## Environment variables
```sh
# Mark this computer as a development machine
#  - user@host will not be shown on zsh prompt
#  - run development tools without warning
NMK_DEVELOPMENT=[true|false]
```


## Integrating with powerline fonts
To make vim-airline display powerline symbols correctly, you need to install a patched font. Instructions can be found in the official powerline [documentation][1], or just download and install prepatched fonts from [powerline-font][2] repository.


All tags in this repository are signed with my public key, run below command to get it.

`gpg --recv-keys 0x28B07F9036262EEF4D5B2B21B837E20D47A47347`


[1]: https://powerline.readthedocs.org/en/latest/installation/linux.html#fonts-installation
[2]: https://github.com/Lokaltog/powerline-fonts
