#!/bin/sh

cur=$(basename "$PWD")

cargo build --target x86_64-pc-windows-gnu && \
cp target/x86_64-pc-windows-gnu/debug/$cur.exe . && \
./$cur.exe