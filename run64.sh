#!/bin/bash

echo "-------------------------------"
uname -a

echo "-------------------------------"
echo "prune64.c"
time ./prune64

echo "-------------------------------"
echo "pqplum64.c"
time ./pqplum64

echo "-------------------------------"
echo "tatami_rust_threaded64.rs"
time ./tatami_rust_threaded64 1000

echo "-------------------------------"
