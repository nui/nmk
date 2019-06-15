FROM rust:slim

RUN apt-get -yq update \
    && apt-get -yq install \
        binutils \
        curl \
        git \
        make \
        musl-tools \
        zsh

RUN rustup target add x86_64-unknown-linux-musl

ENV DIST_DIR=/workspace
