#!/bin/bash

echo "-------------------------------"
uname -a

echo "-------------------------------"
echo "prune64"
time ./prune64

echo "-------------------------------"
echo "pqplum64"
time ./pqplum64

echo "-------------------------------"
echo "tatami_rust_threaded64"
time ./tatami_rust_threaded64 1000

echo "-------------------------------"
