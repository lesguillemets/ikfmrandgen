use ikfmrandgen::{generate_and_filter, to_csv};
use rand::prelude::*;
fn main() {
    let mut rng = thread_rng();
    let seq = generate_and_filter(&mut rng);
    eprintln!("__________________");
    println!("{}", to_csv(&mut rng, seq));
}
