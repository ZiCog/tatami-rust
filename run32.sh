#!/bin/bash

echo "-------------------------------"
uname -a

echo "-------------------------------"
echo "prune32 (C)"
time ./prune32

echo "-------------------------------"
echo "pqplum32 (C + pthreads)"
time ./pqplum32

echo "-------------------------------"
echo "tatami_rust_threaded32 (Rust + rayon)"
time ./tatami_rust_threaded32 200

echo "-------------------------------"
