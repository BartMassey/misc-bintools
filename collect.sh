#!/bin/sh
mkdir -p bin
for COMMAND in shuffle
do
    cargo objcopy --bin $COMMAND --release -- bin/$COMMAND
done
