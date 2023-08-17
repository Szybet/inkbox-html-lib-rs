#!/bin/bash

#rm -rf lib
#mkdir lib
#mkdir lib/include

cargo install cross
RUSTFLAGS='-C link-arg=-s' ~/.cargo/bin/cross build --release --target armv7-unknown-linux-musleabihf
mv target/armv7-unknown-linux-musleabihf/release/libreader_rs.a lib/

cargo install cbindgen
~/.cargo/bin/cbindgen . -o lib/include/libreader-rs.h
