#!/bin/bash

set -ex

cargo clean --target x86_64-pc-windows-gnu
env RUST_LOG=warn 
export CC="/usr/bin/x86_64-w64-mingw32-gcc"
export CXX="/usr/bin/x86_64-w64-mingw32-c++"

RUSTFLAGS='-C link-arg=-s' 
RUST_BACKTRACE=full cargo build -v --target x86_64-pc-windows-gnu

rm -rf output || true
mkdir output || true
cp -f target/x86_64-pc-windows-gnu/debug/file.exe output/.
