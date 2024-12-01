#!/bin/sh

cur=$(basename "$PWD")

rm ./$cur-wasm.zip && /
cargo build --release --target wasm32-unknown-unknown && \
cp target/wasm32-unknown-unknown/release/$cur.wasm . && \
zip -r $cur-wasm.zip index.html $cur.wasm assets/