#!/bin/bash
set -ex

autoreconf -f -i
make distclean || true

export CC=x86_64-w64-mingw32-gcc
export CXX=x86_64-w64-mingw32-c++

export CFLAGS="-I$(pwd)/../libgnurx"
export LDFLAGS="-L$(pwd)/../libgnurx -lshlwapi"


./configure --disable-silent-rules --enable-static=yes --enable-shared=no  --with-PACKAGE=no --prefix=$(pwd)/build --host=x86_64-w64-mingw32

make -j4 install
