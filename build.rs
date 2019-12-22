// build.rs
//
// Generates a global constant array of prime numbers.
//
use crate::primes::Primes;


mod primes;

use std::mem;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;


//type Int = i64;
//const PNUM: usize = 40_000;
//const SMAX: Int = 100_000_000_000;
//const FNUM: usize = 20;

type Int = i32;
const PNUM: usize = 1_300;
const SMAX: Int = 100_000_000;
const FNUM: usize = 10;



fn main() {
    let primes = Primes::new(PNUM);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("constants.rs");
    let mut f = File::create(&dest_path).unwrap();

    if mem::size_of_val(&SMAX) == 4 {
        f.write(b"type Int = i32;\n").unwrap();
    } else {
        f.write(b"type Int = i64;\n").unwrap();
    }


    f.write(b"#[allow(clippy::unreadable_literal)]\n").unwrap();
    f.write(format!("const SMAX: Int = {};\n", SMAX).as_bytes()).unwrap();
    f.write(format!("const FNUM: usize = {};\n", FNUM).as_bytes()).unwrap();
    f.write(format!("pub static PR: [Int; {}] = [\n", PNUM).as_bytes()).unwrap();
    for i in 0..primes.primes.len() {
        f.write(b"    ").unwrap();
        f.write(&primes.primes[i].to_string().into_bytes()).unwrap();
        f.write(b",\n").unwrap();
    }
    f.write(b"];\n").unwrap();



/*
    if p <= SMAX / p + 1 {
        panic!("The maximum prime {} is too small!", p);
    }

    r = 1;
    for i in 0..(FNUM - 1) {
        if primes.primes[i] > SMAX / r + 1 {
            println!("Pr{}={:?}", PNUM, primes.primes[PNUM - 1]);
            return primes;
        }
        r *= primes.primes[i];
    }
    panic!("Distinct primes {} in factorisation too few!", FNUM);
*/

}
