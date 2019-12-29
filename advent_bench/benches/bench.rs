#![feature(test)]
extern crate test;
use adv_bench::solution::*;

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input =
        futures::executor::block_on(async_std::fs::read_to_string("sum-of-primes")).unwrap();
    c.bench_function("p1_me", |b| b.iter(|| run(&input, 100_000)));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
);
criterion_main!(benches);
