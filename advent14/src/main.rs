use std::env;
use std::fs::File;
use std::io::prelude::*;

mod solution;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1]).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("part 1: {}", solution::part1(&contents));
    println!("part 2: {}", solution::part2(&contents));
}
