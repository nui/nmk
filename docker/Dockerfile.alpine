FROM alpine

RUN apk --update add zsh \
    tmux \
    vim \
    python \
    curl

WORKDIR /root

ARG ARCHIVE=nmk.tar.gz

RUN curl -sSL https://storage.googleapis.com/nmk.nuimk.com/nmk.tar.gz | tar -C ~ -xzf -

CMD ["/root/.nmk/bin/nmk", "-l"]
