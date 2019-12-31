use std::env;
use std::mem;
mod error;
mod prune;
use prune::Tatami;

mod queue;

include!(concat!(env!("OUT_DIR"), "/constants.rs"));

use std::convert::TryInto;

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
        println!("Running Rust translation of prune.c...");
        let mut tatami = Tatami::new();
        match tatami.inv(n) {
            Ok(result) => println!("T({})={}", result, n),
            Err(e) => println!("{}", e),
        }

        println!("Running Rust translation of queue.c...");
        let result = queue::tinv(n.try_into().unwrap()); 
        println!("T({})={}", result, n)
    }
}
