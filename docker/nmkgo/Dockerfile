FROM golang

RUN apt-get -yq update \
    && apt-get -yq install \
        binutils \
        curl \
        git \
        make

# install go dep
ARG DEP_INSTALL_PATH=/usr/local/go/bin/dep
ARG DEP_VERSION=v0.5.3
RUN curl -sSLo $DEP_INSTALL_PATH \
        https://github.com/golang/dep/releases/download/${DEP_VERSION}/dep-linux-amd64 \
    && chmod 755 $DEP_INSTALL_PATH

COPY go/ /go/

RUN cd /go \
    && make multiarch

