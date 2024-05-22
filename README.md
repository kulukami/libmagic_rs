# libmagic_rs
rust binding for libmagic https://github.com/file/file

fork from: https://github.com/robo9k/rust-magic-sys

support cross-compile

|host|target|--target|
|-|-|-|
|linux-x64|windows-x64 |--target x86_64-pc-windows-gnu|
|linux-x64|linux-arm64-musl |--target aarch64-unknown-linux-musl|
|linux-x64|linux-x64-musl |--target x86_64-unknown-linux-musl|


# 1. requirements

## 1-1. rust target
```bash
rustup target add aarch64-unknown-linux-musl
rustup target add x86_64-unknown-linux-musl
rustup target add x86_64-pc-windows-gnu
```
## 1-2. debian 
```bash
# x86_64-w64-mingw32-gcc
apt-get install -y autoconf wget gcc-mingw-w64 g++-mingw-w64 mingw-w64-common musl musl-dev musl-tools

# x86_64-linux-musl-gcc
wget https://musl.cc/x86_64-linux-musl-cross.tgz
tar -xf x86_64-linux-musl-cross.tgz
mv x86_64-linux-musl-cross /opt/x86_64-linux-musl
export PATH="$PATH:/opt/x86_64-linux-musl/bin"

# aarch64-linux-musl-gcc
wget https://musl.cc/aarch64-linux-musl-cross.tgz
tar -xf aarch64-linux-musl-cross.tgz
mv aarch64-linux-musl-cross /opt/aarch64-linux-musl
export PATH="$PATH:/opt/aarch64-linux-musl/bin"

```

# 2. Cargo Dependencies
```cargo
[dependencies]
libmagic_rs = { git = "https://github.com/kulukami/libmagic_rs.git", branch = 'main' }
```


# Ideas from 
```txt
https://github.com/microsoft/vcpkg/tree/master/ports/libmagic
https://github.com/robo9k/rust-magic-sys
https://github.com/Nemirtingas/windowscross_vcpkg

```