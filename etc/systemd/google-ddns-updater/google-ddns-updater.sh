#!/bin/sh
LAST_ADDRESS=
ADDRESS=

[ -z "$NAME" ] || [ -z "$USERNAME" ] || [ -z "$PASSWORD" ] || [ -z "$INTERFACE" ] && echo "You must set required environment" 1>&2 && exit 1

alias echo='stdbuf -o0 /bin/echo'

while :; do
    ADDRESS=$(ip -6 addr show dev $INTERFACE scope global primary | sed -e's/^.*inet6 \([^ ]*\)\/.*$/\1/;t;d' | head -n 1)
    if [ "$ADDRESS" != "$LAST_ADDRESS" ] && [ "$#ADDRESS" > 0 ]; then
        curl -s -X POST -H 'cache-control: no-cache' \
            "https://$USERNAME:$PASSWORD@domains.google.com/nic/update?hostname=$NAME&myip=$ADDRESS"
        echo
        echo "updated ipv6 address of $NAME to $ADDRESS"
        echo
    fi
    LAST_ADDRESS=$ADDRESS
    sleep 60
done

