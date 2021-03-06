use criterion::{criterion_group, criterion_main, Criterion};
use scam::bench::load_bench;
use scam::perft::_perft;
use scam::position::{CastleInfo, Position};
use scam::r#move::MoveList;

pub fn pseudolegal_bench(c: &mut Criterion) {
    let bench_pos = load_bench();
    c.bench_function("pseudolegal", |b| {
        b.iter(|| {
            bench_pos
                .iter()
                .fold(0, |acc, (pos, ci)| acc + pos.gen_pseudo_legals(ci).len())
        })
    });
}

pub fn makemove_bench(c: &mut Criterion) {
    let available_moves = load_bench()
        .into_iter()
        .map(|(pos, ci)| {
            let moves = pos.gen_pseudo_legals(&ci);
            (pos, ci, moves)
        })
        .collect::<Vec<(Position, CastleInfo, MoveList)>>();
    c.bench_function("makemove", |b| {
        b.iter(|| {
            available_moves
                .iter()
                .fold(0, |mut acc, (pos, ci, mv_list)| {
                    for i in 0..mv_list.len() {
                        let mv = mv_list.moves[i].0;
                        let mut new_pos = pos.clone();
                        if new_pos.make_move(mv, ci) {
                            acc += 1;
                        }
                    }
                    acc
                })
        })
    });
}

pub fn perft1_bench(c: &mut Criterion) {
    let bench_pos = load_bench();
    c.bench_function("perft1", |b| {
        b.iter(|| {
            bench_pos
                .iter()
                .fold(0, |acc, (pos, ci)| acc + _perft(pos, ci, 1))
        })
    });
}

pub fn perft2_bench(c: &mut Criterion) {
    let bench_pos = load_bench();
    c.bench_function("perft2", |b| {
        b.iter(|| {
            bench_pos
                .iter()
                .fold(0, |acc, (pos, ci)| acc + _perft(pos, ci, 2))
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
