#!/bin/bash

echo "-------------------------------"
uname -a

echo "-------------------------------"
echo "prune64 (C)"
time ./prune64

echo "-------------------------------"
echo "pqplum64 (C + pthreads)"
time ./pqplum64

echo "-------------------------------"
echo "tatami_rust_serial64 (Rust)"
time ./tatami_rust_serial64 1000

echo "-------------------------------"
echo "tatami_rust_threaded64 (Rust + rayon)"
time ./tatami_rust_threaded64 1000

echo "-------------------------------"
