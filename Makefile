LIBDRGN = $(abspath drgn/libdrgn)
LIBDRGN_MAKE = $(LIBDRGN)/Makefile
LIBDRGN_A = $(LIBDRGN)/.libs/libdrgn.a

all: build

build:
	cargo build

libdrgn_a: $(LIBDRGN_A)

$(LIBDRGN_MAKE): $(LIBDRGN_CONF)
	git submodule update --init
	cd $(LIBDRGN);    \
	autoreconf -i -f; \
	./configure

$(LIBDRGN_A): $(LIBDRGN_MAKE)
	cd $(LIBDRGN); make

clean:
	make -C $(LIBDRGN) clean
	cargo clean

run: build
	cargo b --example main
	sudo target/debug/examples/main

