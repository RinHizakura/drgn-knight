HOST_ARCH := $(shell uname -p)

ifeq ($(ARCH), )
    EXPORT_PATH   =
    CARGO_OPT     =
    HOST_OPT      =
else ifeq ($(ARCH), $(HOST_ARCH))
    EXPORT_PATH   =
    CARGO_OPT     =
    HOST_OPT      =
else ifeq ($(ARCH), aarch64)
    LINKER        = CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER
    CROSS_COMPILE = aarch64-unknown-linux-gnu
    EXPORT_PATH   = $(LINKER)=$(CROSS_COMPILE)-gcc
    CARGO_OPT     = --target $(CROSS_COMPILE)
    HOST_OPT      = --host=$(CROSS_COMPILE)
else
    $(error "Non-supported ARCH=$(ARCH)")
endif

all: build

build:
	$(EXPORT_PATH) cargo build $(CARGO_OPT)

LIBDRGN = $(abspath drgn/libdrgn)
libdrgn_a:
	git submodule update --init
	cd $(LIBDRGN);           \
	autoreconf -i -f;        \
	./configure $(HOST_OPT); \
	make clean; make -j$(nporc)

clean:
	cargo clean
