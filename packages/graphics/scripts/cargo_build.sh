#!/bin/bash

rm ./.cargo/config.toml
cp ./.cargo/build.config.toml ./.cargo/config.toml

mkdir ./dist

cargo build --release

# wasm-file optimization
wasm-strip ./target/wasm32-unknown-unknown/release/main.wasm

# another wasm-file optimization and moving to a dist folder
wasm-opt -o ./dist/main.wasm \
    -Oz \
    --zero-filled-memory \
    --enable-threads \
    --enable-bulk-memory \
    ./target/wasm32-unknown-unknown/release/main.wasm

echo "====="
echo "WASM"
echo "size: $(wc -c ./dist/main.wasm | awk '{print $1}') bytes"
echo "====="
