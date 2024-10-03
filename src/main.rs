use ikfmrandgen::{generate_and_filter, print_seq};
use rand::prelude::*;
fn main() {
    let mut rng = thread_rng();
    let seq = generate_and_filter(&mut rng);
    println!("__________________");
    print_seq(seq);
}
