#!/bin/sh

cur=$(basename "$PWD")

cargo build --release --target x86_64-unknown-linux-gnu && \
cp target/x86_64-unknown-linux-gnu/release/$cur ./ && \
tar -zcf $cur-linux.zip $cur assets/*