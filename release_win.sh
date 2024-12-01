#!/bin/sh

cur=$(basename "$PWD")

rm ./$cur-win.zip && \
cargo build --release --target x86_64-pc-windows-gnu && \
cp target/x86_64-pc-windows-gnu/release/$cur.exe ./ && \
zip -r $cur-win.zip $cur.exe assets/
