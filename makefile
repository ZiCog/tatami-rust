CC=clang
CPP=clang++

run: all
	time ./limited
	time ./prune
	time ./queue
	time ./target/release/tatami_rust 200

all: prune limited queue target/release/tatami_rust

target/release/tatami_rust: Cargo.toml build.rs primes.rs  
	RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo build --release --features=use_i32

prune: prune.c
	$(CC) -Wall -O3 -o prune prune.c -march=native -mtune=native

queue: queue.cpp
	$(CPP) -Wall -O3 -o queue queue.cpp -march=native -mtune=native -lpthread

limited: limited.c
	$(CC) -Wall -O3 -o limited limited.c -march=native -mtune=native


clean:
	rm prune limited queue
	cargo clean



