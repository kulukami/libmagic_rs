# Makefile.in for MinGW port of GNU libc 'regex' functions.
# Processed by 'configure' script, to generate 'Makefile'.
# 
PACKAGE = mingw-libgnurx
VERSION = 2.5.1
#
# Adaptation by Keith Marshall <keithmarshall@users.sourceforge.net>
# from original 'Makefile' by Tor Lillqvist <tml@iki.fi>, <tml@novell.com>
#
DLLVERSION = 0
#
# This is free software.  It is provided AS IS, in the hope that it may
# be useful, but WITHOUT WARRANTY OF ANY KIND, not even an IMPLIED WARRANTY
# of MERCHANTABILITY, nor of FITNESS FOR ANY PARTICULAR PURPOSE.
#
# Source files accompanying this Makefile remain copyright of their
# respective authors, or of the Free Software Foundation Inc., as stated
# in individual file headers; all are redistributed with permission, as
# granted by the GNU Lesser General Public License.
#
# Permission is granted to redistribute this software, either "as is" or
# in modified form, under the terms of the GNU Lesser General Public License,
# as published by the Free Software Foundation; either version 2.1, or (at
# your option) any later version.
#
# You should have received a copy of the GNU Lesser General Public License
# along with this software; see the file COPYING.LIB.  If not, write to the
# Free Software Foundation, 51 Franklin St - Fifth Floor, Boston,
# MA 02110-1301, USA.


srcdir = .

prefix = /data00/home/zhangjiacheng.111/opensource/kulu/libmagic_rs/target/x86_64-pc-windows-gnu/debug/build/libmagic_rs-589796a04c43fc38/out/build
exec_prefix = ${prefix}
includedir = ${prefix}/include
libdir = ${exec_prefix}/lib
bindir = ${exec_prefix}/bin
mandir = ${prefix}/share/man

CC = /usr/bin/x86_64-w64-mingw32-gcc -mthreads
CFLAGS = -g -O2 -I ${srcdir}
LDFLAGS =  -Wl,--enable-auto-image-base -Wl,--out-implib,libgnurx.dll.a

SOURCES = $(wildcard ${srcdir}/*.c ${srcdir}/*.h)
OBJECTS = regex.o

MSVCLIB = lib

BINDIST_FILES = libgnurx-$(DLLVERSION).dll
DEVDIST_FILES = libgnurx.dll.a libregex.a 
SRCDIST_FILES = ${srcdir}/configure ${srcdir}/configure.ac ${srcdir}/aclocal.m4 \
  ${srcdir}/pkgid.m4 ${srcdir}/Makefile.in ${srcdir}/README ${srcdir}/COPYING.LIB \
  $(SOURCES) $(wildcard ${srcdir}/*.[37]) ${srcdir}/ChangeLog

ZIPCMD = tar czhf
ZIPEXT = .tar.gz

all: libgnurx-$(DLLVERSION).dll libgnurx.dll.a libregex.a 

Makefile: config.status Makefile.in
	./config.status

config.status: configure
	./config.status --recheck

$(OBJECTS): Makefile

libgnurx-$(DLLVERSION).dll libgnurx.dll.a: $(OBJECTS)
	$(CC) $(CFLAGS) -shared -o libgnurx-$(DLLVERSION).dll $(LDFLAGS) $(OBJECTS)

libregex.a: libgnurx.dll.a
	cp -p libgnurx.dll.a $@

gnurx.lib: libgnurx-$(DLLVERSION).dll
	$(MSVCLIB) -def:libgnurx.def -out:gnurx.lib

install: install-dll install-dev

install-dll:
	mkdir -p ${bindir}
	cp -p $(BINDIST_FILES) ${bindir}

install-dev:
	mkdir -p ${includedir} ${libdir}
	cp -p ${srcdir}/regex.h ${includedir}
	cp -p $(DEVDIST_FILES) ${libdir}
	for s in 3 7; do \
	  mkdir -p ${mandir}/man$$s; \
	  gzip -c ${srcdir}/regex.$$s > ${mandir}/man$$s/regex.$$s.gz; \
	done

dist:	 bindist devdist srcdist
bindist: $(PACKAGE)-$(VERSION)-bin$(ZIPEXT)
devdist: $(PACKAGE)-$(VERSION)-dev$(ZIPEXT)
srcdist: $(PACKAGE)-$(VERSION)-src$(ZIPEXT)

$(PACKAGE)-$(VERSION)-bin$(ZIPEXT): $(BINDIST_FILES)
	rm -rf tmp
	$(MAKE) prefix=`pwd`/tmp --no-print-directory install-dll
	cd tmp; $(ZIPCMD) ../$@ bin
	rm -rf tmp

$(PACKAGE)-$(VERSION)-dev$(ZIPEXT): $(DEVDIST_FILES)
	rm -rf tmp
	$(MAKE) prefix=`pwd`/tmp --no-print-directory install-dev
	cd tmp; $(ZIPCMD) ../$@ .
	rm -rf tmp

$(PACKAGE)-$(VERSION)-src$(ZIPEXT): $(SRCDIST_FILES)
	rm -rf $(PACKAGE)-$(VERSION)
	mkdir $(PACKAGE)-$(VERSION)
	cp -p $(SRCDIST_FILES) $(PACKAGE)-$(VERSION)
	$(ZIPCMD) $@ $(PACKAGE)-$(VERSION)
	rm -rf $(PACKAGE)-$(VERSION)

clean:
	rm -f *~ *.o *.dll *.def *.exp *.lib *.a *.zip
	rm -rf runtime dev $(PACKAGE)-$(VERSION)

distclean: clean
	rm -f Makefile config.status config.log
	rm -rf autom4te.cache

# $RCSfile: Makefile.in,v $Revision: 1.4 $: end of file
