#!/bin/bash

#cargo clean
env RUST_LOG=warn 
cargo clean --target x86_64-unknown-linux-musl
cargo -vv -j6 build --target x86_64-unknown-linux-musl

mkdir output || true
cp -f target/x86_64-unknown-linux-musl/debug/file output/.