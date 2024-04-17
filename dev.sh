#!/bin/bash

#cargo clean
env RUST_LOG=warn cargo -vv build --target x86_64-unknown-linux-musl

mkdir output || true
cp -f target/x86_64-unknown-linux-musl/debug/file output/.
cp -f target/x86_64-unknown-linux-musl/debug/build/libmagic_rs-*/out/build/share/misc/magic.mgc output/.
cp -f target/x86_64-unknown-linux-musl/debug/build/libmagic_rs-*/out/build/bin/file output/file_static