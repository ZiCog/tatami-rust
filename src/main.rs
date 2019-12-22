
mod prune;
mod error;

use prune::Tatami;

fn main() {
//    let n = 1000;
    let n = 200;

    let mut tatami = Tatami::new();
    match tatami.inv(n) {
        Ok(result) => println!("T({})={}", result, n),
        Err(e) => println!("{}", e),
    }
}
