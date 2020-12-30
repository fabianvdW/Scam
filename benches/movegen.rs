use criterion::{black_box, criterion_group, criterion_main, Criterion};
use scam::test;

pub fn pseudolegal_bench(c: &mut Criterion) {
    c.bench_function("pseudolegal", |b| b.iter(|| test(black_box(20))));
}
pub fn makemove_bench(c: &mut Criterion) {
    c.bench_function("makemove", |b| b.iter(|| 0));
}
pub fn perft_bench(c: &mut Criterion) {
    c.bench_function("perft", |b| b.iter(|| 0));
}

criterion_group!(benches, pseudolegal_bench, makemove_bench, perft_bench);
criterion_main!(benches);
