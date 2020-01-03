use std::env;
use std::mem;
mod error;

#[cfg(feature = "serial")]
mod prune;
#[cfg(feature = "serial")]
use prune::Tatami;

#[cfg(feature = "threaded")]
mod queue;

include!(concat!(env!("OUT_DIR"), "/constants.rs"));

#[cfg(feature = "threaded")]
use std::convert::TryInto;

#[cfg(feature = "threaded")]
fn do_it(n: PrimeType) {
    println!("Running Rust translation of queue.c...");
    let result = queue::tinv(n.try_into().unwrap());
    println!("T({})={}", result, n)
}

#[cfg(feature = "serial")]
fn do_it(n: PrimeType) {
    println!("Running Rust translation of prune.c...");
    let mut tatami = Tatami::new();
    match tatami.inv(n) {
        Ok(result) => println!("T({})={}", result, n),
        Err(e) => println!("{}", e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Did you forget an argument?");
        return;
    }

    println!("Using {} bit integers.", mem::size_of_val(&SMAX) * 8);
    println!("PNUM = {}", PR.len());
    println!("FNUM = {}", FNUM);
    println!("SMAX = {}", SMAX);

    println!("Pr({})={}", PR.len(), PR.last().unwrap());

    if let Ok(n) = args[1].parse::<PrimeType>() {
        do_it(n);
    }
}
