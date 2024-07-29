LIBDRGN = $(abspath drgn/libdrgn)
LIBDRGN_A = $(LIBDRGN)/.libs/libdrgn.a

all: build

build: $(LIBDRGN_A)
	cargo build

$(LIBDRGN):
	git submodule update --init

$(LIBDRGN_A): $(LIBDRGN)
	cd $(LIBDRGN);     \
	autoreconf -i -f;  \
	./configure;       \
	make

clean:
	make -C $(LIBDRGN) clean
	cargo clean

run: build
	sudo cargo r --example main
