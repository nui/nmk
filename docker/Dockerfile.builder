FROM alpine

RUN apk --update add coreutils git python3

WORKDIR /root

RUN git clone https://github.com/nuimk/nmk.git ~/.nmk

ENTRYPOINT ["python3", "/root/.nmk/etc/build.py", "--no-upload", "--branch", "master"]

