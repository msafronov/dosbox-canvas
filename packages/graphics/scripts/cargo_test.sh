#!/bin/bash

# wasm-bindgen-test-runner failed with "atomics" and "bulk-memory" features
# rustflags by profile are only nightly: https://github.com/rust-lang/cargo/pull/10217
# so there is a temporary solution: replace main ".cargo/config.toml" with test-only config:

rm ./.cargo/config.toml
cp ./.cargo/test.config.toml ./.cargo/config.toml

# run tests

cargo test --features unit_test --workspace