use criterion::{black_box, criterion_group, criterion_main, Criterion};
use scam::test;

pub fn pseudolegal_bench(c: &mut Criterion) {
    c.bench_function("pseudolegal", |b| b.iter(|| test(black_box(20))));
}

pub fn makemove_bench(c: &mut Criterion) {
    c.bench_function("makemove", |b| b.iter(|| 0));
}

pub fn perft1_bench(c: &mut Criterion) {
    c.bench_function("perft1", |b| b.iter(|| 0));
}

pub fn perft2_bench(c: &mut Criterion) {
    c.bench_function("perft2", |b| b.iter(|| 0));
}

pub fn perft3_bench(c: &mut Criterion) {
    c.bench_function("perft3", |b| b.iter(|| 0));
}

criterion_group!(
    benches,
    pseudolegal_bench,
    makemove_bench,
    perft1_bench,
    perft2_bench,
    perft3_bench
);
criterion_main!(benches);
