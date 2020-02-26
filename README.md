# tatami-rust
A solution to the Project Euler problem number 256 : Tatami-Free Rooms, in Rust.

## Build

This program can be built to use 32 or 64 bit integers. A trade off between performance and problem size capability.

Use: 

    $ RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo build --release --features=use_u32

Or:

    $ RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo build --release --features=use_u32

There is a make file to build and time the Rust and the C versions it is derived from:

    $ make clean
    $ make

Note: This crashed cargo/rustc on the Raspberry Pi under Raspbian. However I used raspbian-nspawn-64 to get a 64 bit shell (when one has booted a 64 bit kernel) where it all compiles and runs just fine. See here for instructions on raspbian-nspawn-64 https://www.raspberrypi.org/forums/viewtopic.php?p=1566212#p1566212


## Run

A 64 bit build can handle an 'n' argument up to 1000

    $ time target/release/tatami_rust 1000
    Using 64 bit integers.
    PNUM = 40000
    FNUM = 20
    SMAX = 100000000000
    T(63405342000)=1000

    real    4m50.396s
    user    4m49.500s
    sys     0m0.016s

A 32 bit build cannot:

    $ time target/release/tatami_rust 1000
    Using 32 bit integers.
    PNUM = 1300
    FNUM = 10
    SMAX = 100000000
    borked

    real    0m0.239s
    user    0m0.219s
    sys     0m0.000s

But up to 200 is OK:

    $ time target/release/tatami_rust 200
    Using 32 bit integers.
    PNUM = 1300
    FNUM = 10
    SMAX = 100000000
    T(85765680)=200

    real    0m0.638s
    user    0m0.609s
    sys     0m0.000s

## Original C codes:

The original solutions by E.J.Olsen in C are included here: limited.c for and prune.c. The Rust version is derived from prune.c.
See link below for his repository.

    $ gcc -Wall -O3 -o limited limited.c -march=native -mtune=native
    $ time ./limited
    T(85765680)=200

    real    0m0.915s
    user    0m0.891s
    sys     0m0.016s

    $ gcc -Wall -O3 -o prune prune.c -march=native -mtune=native
    $ time ./prune
    Pr(40000)=479909
    T(63405342000)=1000

    real    4m46.280s
    user    4m46.172s
    sys     0m0.016s


## Credits:

Eric Olson - For the original single threaded prune.c.
http://fractal.math.unr.edu/~ejolson/pi/tatami/src/

Jean M. Cyr - For the threaded queue.c version.

alice - Suggested using rayon scope and other helpful hints. 
https://users.rust-lang.org/t/yes-at-last-my-rust-is-faster-than-c/36100/21

All contributors to this little Tatami cosing challenge discussion thread on the Raspberry Pi forum:
https://www.raspberrypi.org/forums/viewtopic.php?f=31&t=257317


