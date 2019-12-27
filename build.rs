// build.rs
//
// Generates constants.rs containing type and constant definitions.
//
use crate::primes::Primes;

mod primes;

use std::env;
use std::fs::File;
use std::io::Write;
use std::mem;
use std::path::Path;

#[cfg(feature = "use_i32")]
mod defs {
    pub type Int = i32;
    pub const PNUM: usize = 1_300;
    pub const SMAX: Int = 100_000_000;
    pub const FNUM: usize = 10;
}
#[cfg(feature = "use_i64")]
mod defs {
    pub type Int = i64;
    pub const PNUM: usize = 40_000;
    pub const SMAX: Int = 100_000_000_000;
    pub const FNUM: usize = 20;
}
use defs::{Int, FNUM, PNUM, SMAX};

fn main() {
    let primes = Primes::new(PNUM);

    if let Some(last) = primes.primes.last() {
        if *last as Int <= SMAX / *last as Int + 1 {
            panic!("The maximum prime {} is too small!", last);
        }
    }

    let mut r: Int = 1;
    let mut ok = false;
    for i in 0..(FNUM - 1) {
        if primes.primes[i] as Int > SMAX / r + 1 {
            println!("Pr{}={:?}", PNUM, primes.primes[PNUM - 1]);
            ok = true;
        }
        r *= primes.primes[i] as Int;
    }
    if !ok {
        panic!("Distinct primes {} in factorisation too few!", FNUM);
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("constants.rs");
    let mut f = File::create(&dest_path).unwrap();

    if mem::size_of_val(&SMAX) == 4 {
        f.write(b"type Int = u32;\n").unwrap();
    } else {
        f.write(b"type Int = i64;\n").unwrap();
    }

    f.write(b"#[allow(clippy::unreadable_literal)]\n").unwrap();
    f.write(format!("const SMAX: Int = {};\n", SMAX).as_bytes())
        .unwrap();
    f.write(format!("const FNUM: usize = {};\n", FNUM).as_bytes())
        .unwrap();
    f.write(format!("pub static PR: [Int; {}] = [\n", PNUM).as_bytes())
        .unwrap();
    for i in 0..primes.primes.len() {
        f.write(b"    ").unwrap();
        f.write(format!("{}", primes.primes[i]).as_bytes()).unwrap();
        f.write(b",\n").unwrap();
    }
    f.write(b"];\n").unwrap();
}
