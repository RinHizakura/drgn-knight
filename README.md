# drgn-knight

## Introduction

drgn-knight is a Rust library which enable to access kernel object through
"/proc/kcore". It is built on top of [drgn](https://github.com/osandov/drgn)
with only a restricted set of interfaces.

## Build

### drgn

In order to compile drgn from source for libdrgn, you will need these dependencies:

```
$ sudo apt install autoconf automake check gcc git liblzma-dev libelf-dev libdw-dev libtool make pkgconf python3 python3-dev python3-pip python3-setuptools zlib1g-dev
```


The vmlinux file for you kernel is required for the debug information. Reading
[Getting Debugging Symbols](https://github.com/osandov/drgn/blob/main/docs/getting_debugging_symbols.rst)
to know how you can get it.

### drgn-knight

```
$ git submodule update --init
$ make
```
