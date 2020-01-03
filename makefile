CC=gcc
CPP=g++

RUST_SOURCES=build.rs defs.rs primes.rs src/error.rs src/main.rs src/prune.rs src/queue.rs

all: prune32 prune64 limited queue pqplum32 pqplum64 tatami_rust_serial32 tatami_rust_serial64 tatami_rust_threaded32 tatami_rust_threaded64


tatami_rust_serial32: $(RUST_SOURCES) 
	RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo build --release --features=use_u32 --features=serial
	cp target/release/tatami_rust tatami_rust_serial32

tatami_rust_serial64: $(RUST_SOURCES)
	RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo build --release --features=use_u64 --features=serial
	cp target/release/tatami_rust tatami_rust_serial64

tatami_rust_threaded32: $(RUST_SOURCES)
	RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo build --release --features=use_u32 --features=threaded
	cp target/release/tatami_rust tatami_rust_threaded32

tatami_rust_threaded64: $(RUST_SOURCES)
	RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo build --release --features=use_u64 --features=threaded
	cp target/release/tatami_rust tatami_rust_threaded64

prune64: prune.c
	$(CC) -Wall -O3 -DT_S=1000 -o prune64 prune.c -mtune=native

prune32: prune.c
	$(CC) -Wall -O3 -DT_S=200 -o prune32 prune.c  -mtune=native

pqplum64: pqplum.c
	$(CC) -Wall -O3 -DT_S=1000 -o pqplum64 pqplum.c  -mtune=native  -lpthread -lm

pqplum32: pqplum.c
	$(CC) -Wall -O3 -DT_S=200 -o pqplum32 pqplum.c  -mtune=native  -lpthread -lm

queue: queue.cpp
	$(CPP) -Wall -O3 -o queue queue.cpp  -mtune=native -lpthread

limited: limited.c
	$(CC) -Wall -O3 -o limited limited.c  -mtune=native

clean:
	rm -f prune32 prune64 limited queue pqplum32 pqplum64 tatami_rust_serial32 tatami_rust_serial64 tatami_rust_threaded32 tatami_rust_threaded64
	cargo clean



