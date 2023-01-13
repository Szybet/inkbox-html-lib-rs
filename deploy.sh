#!/bin/bash

rm -rf lib
mkdir lib
mkdir lib/include

cargo install cross
~/.cargo/bin/cross build --release --target armv7-unknown-linux-musleabihf
mv target/armv7-unknown-linux-musleabihf/release/libreader_rs.a lib/
# If it's as a submodule
cp lib/libreader_rs.a ../prebuild

cargo install cbindgen
~/.cargo/bin/cbindgen . -o lib/include/libreader-rs.h
