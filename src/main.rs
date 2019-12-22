use std::env;
mod prune;
mod error;
use prune::Tatami;

include!(concat!(env!("OUT_DIR"), "/constants.rs"));

fn main() {
//    let n = 1000;
    let n = 200;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Did you forget an argument?");
        return
    }
    if let Ok(n) = args[1].parse::<Int>() {
        let mut tatami = Tatami::new();
        match tatami.inv(n) {
            Ok(result) => println!("T({})={}", result, n),
            Err(e) => println!("{}", e),
        }
    }
}
