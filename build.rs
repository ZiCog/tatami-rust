// build.rs
//
// Generates a global constant array of prime numbers.
//
use crate::primes::Primes;

mod constants;

mod primes;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let primes = Primes::new();

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("prime.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write(b"pub const PR: [i64; 40_000] = [\n").unwrap();
    for i in 0..primes.primes.len() {
        f.write(b"    ").unwrap();
        f.write(&primes.primes[i].to_string().into_bytes()).unwrap();
        f.write(b",\n").unwrap();
    }
    f.write(b"];\n").unwrap();
}
