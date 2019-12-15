use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::time::{Duration, Instant};

mod solution;

#[inline(always)]
fn timed<T>(f: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let result = f();
    let end = Instant::now();
    (result, end - start)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file = File::open(&args[1]).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let (part1_solution, part1_duration) = timed(|| solution::part1(&contents));
    let (part2_solution, part2_duration) = timed(|| solution::part2(&contents));
    println!("part 1: {} timed: {:?}", part1_solution, part1_duration);
    println!("part 2: {} timed: {:?}", part2_solution, part2_duration);
}
