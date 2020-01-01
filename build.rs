// build.rs
//
// Generates constants.rs containing type and constant definitions.
//
use crate::primes::Primes;

mod defs;
mod primes;

use std::env;
use std::fs::File;
use std::io::Write;
use std::mem;
use std::path::Path;

use defs::defs::{Int, FNUM, PNUM, SMAX};

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
        f.write_all(b"use std::sync::atomic::AtomicU32;\n").unwrap();
        f.write_all(b"type PrimeType = u32;\n").unwrap();
        f.write_all(b"type AtomicType = AtomicU32;\n").unwrap();
    } else {
        f.write_all(b"use std::sync::atomic::AtomicU64;\n").unwrap();
        f.write_all(b"type PrimeType = u64;\n").unwrap();
        f.write_all(b"type AtomicType = AtomicU64;\n").unwrap();
    }

    f.write_all(b"#[allow(clippy::unreadable_literal)]\n")
        .unwrap();
    f.write_all(format!("const SMAX: PrimeType = {};\n", SMAX).as_bytes())
        .unwrap();
    f.write_all(format!("const FNUM: usize = {};\n", FNUM).as_bytes())
        .unwrap();
    f.write_all(format!("pub static PR: [PrimeType; {}] = [\n", PNUM).as_bytes())
        .unwrap();
    for i in 0..primes.primes.len() {
        f.write_all(b"    ").unwrap();
        f.write_all(format!("{}", primes.primes[i]).as_bytes())
            .unwrap();
        f.write_all(b",\n").unwrap();
    }
    f.write_all(b"];\n").unwrap();
}
