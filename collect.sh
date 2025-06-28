#!/bin/sh
mkdir -p bin
for COMMAND in shuffle ipaddr
do
    cargo objcopy --bin $COMMAND --release -- bin/$COMMAND
done
