FROM debian:8
COPY setup /tmp/nmk-setup
RUN export HOME=/root DEBIAN_FRONTEND=noninteractive \
    && apt-get -qq update \
    && apt-get -qq upgrade \
    && apt-get -qq install \
        curl \
        dnsutils \
        iputils-ping \
        man \
        net-tools \
        rsync \
        sudo \
        wget \
    # debian locale
    && apt-get -qq install locales \
    && echo "en_US.UTF-8 UTF-8" >> /etc/locale.gen \
    && locale-gen 'en_US.utf8' \
    && update-locale LANG=en_US.UTF-8 \
    && apt-get -qq install exuberant-ctags git tmux vim-nox zsh \
    && /tmp/nmk-setup/automate \
    && gpg --keyserver hkp://keys.gnupg.net --recv-keys 0x66939600 \
    && rm -rf /tmp/nmk-setup \
    && rm -rf /var/lib/apt/lists/*
CMD ["/root/.nmk/bin/nmk"]
WORKDIR /root/.nmk
