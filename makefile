run: all
	time ./limited
	time ./prune
	time ./target/release/tatami_rust 200

all: prune limited target/release/tatami_rust

target/release/tatami_rust: Cargo.toml build.rs primes.rs  
	RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo build --release --features=use_i32


prune: prune.c
	gcc -Wall -O3 -o prune prune.c -march=native -mtune=native

limited: limited.c
	gcc -Wall -O3 -o limited limited.c -march=native -mtune=native


clean:
	rm prune limited target/release/tatami_rust target/debug/tatami_rust


