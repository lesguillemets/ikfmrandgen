use ikfmrandgen::generate_and_filter;
use rand::prelude::*;
fn main() {
    let mut rng = thread_rng();
    let seq = generate_and_filter(&mut rng);
    println!("__________________");
    for trial in &seq {
        println!("{:?}", trial);
    }
}
