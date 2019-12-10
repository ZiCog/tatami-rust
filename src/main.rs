mod constants;
mod prune;

use prune::Tatami;

fn main() {
    let n = 1000;
    let mut tatami = Tatami::new();
    let result = tatami.inv(n);
    println!("T({})={}", result, n);
}
