#!/bin/sh
mkdir -p bin
for COMMAND in shuffle ipaddr ipname
do
    cargo objcopy --bin $COMMAND --release -- bin/$COMMAND
done
