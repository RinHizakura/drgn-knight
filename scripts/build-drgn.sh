#!/usr/bin/env bash
set -e

cd drgn/libdrgn
autoreconf -i -f
./configure
make
