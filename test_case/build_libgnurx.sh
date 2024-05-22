#!/bin/bash

wget https://downloads.sourceforge.net/mingw/Other/UserContributed/regex/mingw-regex-2.5.1/mingw-libgnurx-2.5.1-src.tar.gz

tar -xf mingw-libgnurx-2.5.1-src.tar.gz

mv mingw-libgnurx-2.5.1  libgnurx

cd libgnurx

./configure --host=x86_64-w64-mingw32

make