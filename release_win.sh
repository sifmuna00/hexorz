#!/bin/sh

cur=$(basename "$PWD")

cargo build --release --target x86_64-pc-windows-gnu && \
cp target/x86_64-pc-windows-gnu/release/$cur.exe ./ && \
tar -c -a -f $cur-win.zip $cur.exe assets/* \
