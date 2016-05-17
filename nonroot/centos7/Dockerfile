FROM centos:7

RUN yum -y update
RUN yum -y install file gcc kernel-devel make ncurses-devel

# compile libevent
RUN mkdir /build
WORKDIR /build

ARG LOCAL_PREFIX=/root/.nmk/local

RUN curl -sSL https://github.com/libevent/libevent/releases/download/release-2.0.22-stable/libevent-2.0.22-stable.tar.gz | tar xz
RUN cd libevent-2.0.22-stable \
    && ./configure --prefix=$LOCAL_PREFIX \
    && make \
    && make install

# compile tmux
RUN curl -sSL https://github.com/tmux/tmux/releases/download/2.2/tmux-2.2.tar.gz | tar xz
RUN cd tmux-2.2 \
    && ./configure --prefix=$LOCAL_PREFIX CFLAGS="-I$LOCAL_PREFIX/include -I$LOCAL_PREFIX/include/ncurses" LDFLAGS="-L$LOCAL_PREFIX/lib -L$LOCAL_PREFIX/include/ncurses -L$LOCAL_PREFIX/include" \
    && CPPFLAGS="-I$LOCAL_PREFIX/include -I$LOCAL_PREFIX/include/ncurses" LDFLAGS="-static -L$LOCAL_PREFIX/include -L$LOCAL_PREFIX/include/ncurses -L$LOCAL_PREFIX/lib" make \
    && make install

# compile zsh
RUN curl -OL http://jaist.dl.sourceforge.net/project/zsh/zsh/5.2/zsh-5.2.tar.gz \
    && sha256sum zsh-5.2.tar.gz | grep -q fa924c534c6633c219dcffdcd7da9399dabfb63347f88ce6ddcd5bb441215937 \
    && tar xf zsh-5.2.tar.gz
RUN cd zsh-5.2 \
    && ./configure --prefix=$LOCAL_PREFIX --without-tcsetpgrp CFLAGS="-I$LOCAL_PREFIX/include -I$LOCAL_PREFIX/include/ncurses" LDFLAGS="-L$LOCAL_PREFIX/lib -L$LOCAL_PREFIX/include/ncurses -L$LOCAL_PREFIX/include" \
    && CPPFLAGS="-I$LOCAL_PREFIX/include -I$LOCAL_PREFIX/include/ncurses" LDFLAGS="-static -L$LOCAL_PREFIX/include -L$LOCAL_PREFIX/include/ncurses -L$LOCAL_PREFIX/lib" make \
    && make install

RUN cd $LOCAL_PREFIX && tar caf /local.tar.gz *
