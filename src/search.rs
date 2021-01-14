use crate::eval::eval;
use crate::position::*;
use crate::r#move::*;
use crate::types::*;

use std::time::Instant;

fn printable_score(score: Score) -> (&'static str, Score) {
    if score >= MATE_IN_MAX {
        if score > 0 {
            ("mate", (MATE - score) / 2 + 1)
        } else {
            ("mate", -(MATE + score) / 2)
        }
    } else {
        ("cp", score)
    }
}

fn print_thinking(depth: i32, score: Score, start: Instant) {
    let elapsed = start.elapsed().as_millis();
    let (score_type, score) = printable_score(score);

    println!(
        "info depth {} score {} {} time {}",
        depth, score_type, score, elapsed
    );
}

pub fn start_search(pos: &Position, ci: &CastleInfo) {
    let start_time = Instant::now();
    let max_depth = 6;

    let mut best_move = Move::new(0, 0, 0, None);

    for d in 0..=max_depth {
        let (mv, score) = search(pos, ci, d, 0);
        best_move = mv;

        print_thinking(d, score, start_time);
    }

    println!("bestmove {}", best_move.to_str(ci));
}

fn search(pos: &Position, ci: &CastleInfo, depth: i32, ply: i32) -> (Move, Score) {
    let mut best_score = -MATE;
    let mut best_move = Move::new(0, 0, 0, None);

    if depth == 0 {
        return (best_move, eval(pos));
    }

    let mut move_count = 0;

    for mv in pos.gen_pseudo_legals(ci) {
        let mut new_pos = pos.clone();
        if !new_pos.make_move(mv, ci) {
            continue;
        }

        move_count += 1;

        let (_, mut score) = search(&new_pos, ci, depth - 1, ply + 1);
        score = -score;

        if score > best_score {
            best_score = score;
            best_move = mv;
        }
    }

    if move_count == 0 {
        best_score = if pos.in_check(pos.ctm) {
            -MATE + ply
        } else {
            0
        };
    }

    (best_move, best_score)
}
