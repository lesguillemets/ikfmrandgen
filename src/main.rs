use ikfmrandgen::generate_seq;
use rand::prelude::*;
fn main() {
    let mut rng = thread_rng();
    let seq = generate_seq(&mut rng);
    println!("__________________");
    for trial in &seq {
        println!("{:?}", trial);
    }
}
