#!/usr/bin/env bash
set -e

# FIXME: Improve the flow to get the build taget
cd drgn
python3 setup.py build
cp build/temp.linux-x86_64-3.10/.libs/libdrgnimpl.a ..
