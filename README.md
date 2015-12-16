# My linux working environment settings

## Installation

```sh
# SYSTEM DEPENDENCIES
# server:
    sudo apt install git tmux vim-nox zsh
# desktop:
    sudo apt install git tmux vim-gtk xclip zsh

# NMK
# by automated script
    wget -qO- https://github.com/nuimk/nmk/raw/master/setup/automate | zsh
# or basic github checkout
    git clone --recursive https://github.com/nuimk/nmk.git ~/.nmk
    ~/.nmk/post-clone
    ~/.nmk/bin/nmk
    ~/.nmk/vim/update-plugins
# or by download from
    wget https://dl.dropboxusercontent.com/u/20621424/nmk/nmk.tar.xz
```


## Terminal setup

To use 256 colors, Set `TERM` environment variable to `xterm-256color`.

**Konsole** (KDE Terminal):
- Right click and choose `Edit Current Profiles`
- Click edit button next to environment label
- Change `TERM=xterm` to `TERM=xterm-256color`


## Environment variables

```sh
# Mark this computer as a development machine
#  - user@host will not be shown on zsh prompt
#  - run development tools without warning
NMK_DEVELOPMENT=[true|false]
```


## Integrating with powerline fonts

To make vim-airline display powerline symbols correctly, you need to install a patched font. Instructions can be found in the official powerline [documentation][1], or just download and install prepatched fonts from [powerline-font][2] repository.


## Vim and zsh without tmux

Append below lines to `~/.profile`

```sh
export NMK_DIR=${NMK_DIR:-$HOME/.nmk}
export NMK_DEVELOPMENT=true

export VIMINIT='source $NMK_DIR/vim/init.vim'
export ZDOTDIR=${ZDOTDIR:-$NMK_DIR/zsh}
PATH=$NMK_DIR/bin:$PATH
```

Log out and log back in.

All tags in this repository are signed with my public key, run below command to get it.

`gpg --recv-keys 0xD6F342D866939600`



[1]: https://powerline.readthedocs.org/en/latest/installation/linux.html#fonts-installation
[2]: https://github.com/Lokaltog/powerline-fonts
