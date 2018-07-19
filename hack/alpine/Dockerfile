FROM alpine

RUN apk --update add zsh \
    tmux \
    vim \
    python \
    curl

WORKDIR /root

ARG ARCHIVE=nmk.tar.gz

RUN curl -o $ARCHIVE nmk.nuimk.com/$ARCHIVE \
    && tar xf $ARCHIVE

