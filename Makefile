all: build

build:
	cargo build

clean:
	cargo clean

run: build
	sudo cargo r --example main
