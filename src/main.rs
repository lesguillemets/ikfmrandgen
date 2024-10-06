use chrono::Local;
use ikfmrandgen::{generate_and_filter, to_csv};
use rand::prelude::*;
use std::env;
use std::fs;

fn generate_once<R: Rng>(rng: &mut R) {
    let seq = generate_and_filter(rng);
    eprintln!("__________________");
    println!("{}", to_csv(rng, seq));
}

fn generate_n<R: Rng>(rng: &mut R, n: usize) {
    eprintln!("generating {} files....", n);
    let current = Local::now();
    let timestamp = format!("{}", current.format("%Y%m%d-%H%M%S"));
    for i in 0..n {
        let seq = generate_and_filter(rng);
        let text = to_csv(rng, seq);
        // FIXME: hard coded to 3 chars
        let file_name = format!("seq_{}_{:0>3}.csv", timestamp, i);
        fs::write(&file_name, &text).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rng = thread_rng();
    if args.len() == 1 {
        // run without any argument
        generate_once(&mut rng);
        return;
    }
    if let Ok(n) = args[1].parse::<usize>() {
        generate_n(&mut rng, n);
    } else {
        eprintln!(
            "error: give no argument to generate and print once; otherwise give N / given {:?}",
            &args
        );
    }
}
