FROM alpine

RUN apk --update add \
    coreutils \
    curl \
    git \
    python3 \
    tmux \
    vim \
    zsh

WORKDIR /root

RUN git clone https://github.com/nuimk/nmk.git ~/.nmk

ENTRYPOINT ["python3", "/root/.nmk/etc/build.py", "--no-upload", "--branch", "master"]

