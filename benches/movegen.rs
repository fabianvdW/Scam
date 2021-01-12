use criterion::{criterion_group, criterion_main, Criterion};
use scam::bench::load_bench;
use scam::perft::_perft;
use scam::position::Position;
use scam::r#move::MoveList;

pub fn pseudolegal_bench(c: &mut Criterion) {
    let bench_pos = load_bench();
    c.bench_function("pseudolegal", |b| {
        b.iter(|| {
            bench_pos
                .iter()
                .fold(0, |acc, pos| acc + pos.gen_pseudo_legals().len())
        })
    });
}

pub fn makemove_bench(c: &mut Criterion) {
    let mut available_moves = load_bench()
        .into_iter()
        .map(|pos| {
            let moves = pos.gen_pseudo_legals();
            (pos, moves)
        })
        .collect::<Vec<(Position, MoveList)>>();
    c.bench_function("makemove", |b| {
        b.iter(|| {
            available_moves
                .iter_mut()
                .fold(0, |mut acc, (pos, mv_list)| {
                    for i in 0..mv_list.len() {
                        let mv = mv_list.moves[i].0;
                        if pos.make_move(mv) {
                            acc += 1;
                            pos.unmake_move();
                        }
                    }
                    acc
                })
        })
    });
}

pub fn perft1_bench(c: &mut Criterion) {
    let mut bench_pos = load_bench();
    c.bench_function("perft1", |b| {
        b.iter(|| {
            bench_pos
                .iter_mut()
                .fold(0, |acc, pos| acc + _perft(pos, 1))
        })
    });
}

pub fn perft2_bench(c: &mut Criterion) {
    let mut bench_pos = load_bench();
    c.bench_function("perft2", |b| {
        b.iter(|| {
            bench_pos
                .iter_mut()
                .fold(0, |acc, pos| acc + _perft(pos, 2))
        })
    });
}

criterion_group!(
    benches,
    pseudolegal_bench,
    makemove_bench,
    perft1_bench,
    perft2_bench,
);
criterion_main!(benches);
