FROM ubuntu:16.04
# use old version of glibc to ensure binary compatibility

RUN apt-get -yq update \
    && apt-get -yq install \
        binutils \
        build-essential \
        curl \
        gcc-arm-linux-gnueabihf \
        git \
        libssl-dev \
        make \
        musl-tools \
        pigz \
        pixz \
        zsh \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path \
    && . /root/.cargo/env \
    && rustup target add \
        armv7-unknown-linux-gnueabihf \
        x86_64-unknown-linux-musl \
    && cargo install sccache

ENV DIST_DIR=/workspace
ENV PATH=/root/.cargo/bin:${PATH}
