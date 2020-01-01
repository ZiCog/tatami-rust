#!/bin/bash

RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo build --release --features=use_u32 --features=serial
cp target/release/tatami_rust tatami_rust_serial32

RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo build --release --features=use_u64 --features=serial
cp target/release/tatami_rust tatami_rust_serial64

RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo build --release --features=use_u32 --features=threaded
cp target/release/tatami_rust tatami_rust_threaded32

RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo build --release --features=use_u64 --features=threaded
cp target/release/tatami_rust tatami_rust_threaded64


time  ./tatami_rust_serial32 200
time  ./tatami_rust_serial64 200
time  ./tatami_rust_threaded32 200
time  ./tatami_rust_threaded64 200


