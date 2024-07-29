all: build

build:
	git submodule update --init
	cargo build

clean:
	cargo clean

run: build
	sudo cargo r --example main
