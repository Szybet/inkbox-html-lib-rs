#!/bin/bash
~/.cargo/bin/cross build --release --target armv7-unknown-linux-musleabihf
~/.cargo/bin/cbindgen . -o libreader-rs.h
