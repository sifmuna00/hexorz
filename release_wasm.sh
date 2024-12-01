#!/bin/sh

cur=$(basename "$PWD")

cargo build --release --target wasm32-unknown-unknown && \
cp target/wasm32-unknown-unknown/release/$cur.wasm . && \
tar -cf $cur-wasm.zip index.html $cur.wasm assets/*