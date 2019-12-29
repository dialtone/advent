#![feature(test)]
extern crate test;
use adv_bench::solution::*;

use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input =
        futures::executor::block_on(async_std::fs::read_to_string("sum-of-primes")).unwrap();
    c.bench_function("sum-of-primes", |b| b.iter(|| run(&input, vec![100_000])));
    let input = futures::executor::block_on(async_std::fs::read_to_string("factor")).unwrap();
    c.bench_function("factor-small", |b| {
        b.iter(|| run(&input, vec![2_147_483_647]))
    });
    c.bench_function("factor-big", |b| {
        b.iter(|| run(&input, vec![19_201_644_899]))
    });

    let input = futures::executor::block_on(async_std::fs::read_to_string("isqrt")).unwrap();
    c.bench_function("isqrt", |b| b.iter(|| run(&input, vec![130])));

    let input = futures::executor::block_on(async_std::fs::read_to_string("ackermann")).unwrap();
    c.bench_function("ackermann", |b| b.iter(|| run(&input, vec![3, 6])));

    let input = futures::executor::block_on(async_std::fs::read_to_string("divmod")).unwrap();
    c.bench_function("divmod", |b| b.iter(|| run(&input, vec![1024, 3])));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
);
criterion_main!(benches);
