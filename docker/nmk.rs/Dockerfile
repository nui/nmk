FROM ubuntu:14.04
# use old version of glibc to ensure binary compatibility

RUN apt-get -yq update \
    && apt-get -yq install \
        binutils \
        build-essential \
        curl \
        gcc-arm-linux-gnueabihf \
        git \
        make \
        musl-tools \
        zsh \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path \
    && . /root/.cargo/env \
    && rustup target add \
        # x86_64-unknown-linux-musl \
        armv7-unknown-linux-gnueabihf \
    && rustup show

COPY cargo-config /root/.cargo/config

ENV DIST_DIR=/workspace
ENV PATH=/root/.cargo/bin:${PATH}
