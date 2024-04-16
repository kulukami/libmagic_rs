#!/bin/bash

cargo clean
env RUST_LOG=warn cargo -vv build --target x86_64-unknown-linux-musl