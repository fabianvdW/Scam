use crate::eval::eval;
use crate::position::*;
use crate::r#move::*;
use crate::thread::Thread;
use crate::types::*;

use crate::transposition::{FLAG_EXACT, FLAG_LOWER, FLAG_UPPER};
use std::sync::atomic::Ordering;
use std::time::Instant;

pub const MAX_DEPTH: u8 = 100;
pub const CHECKUP_NODES: u64 = 1 << 15;

#[derive(Clone)]
pub struct Limits {
    pub start: Instant,
    pub spend: u128,

    pub time: u128,
    pub inc: u128,

    pub movetime: u128,
    pub moves_to_go: u128,

    pub depth: u8,
    pub mate: i32,

    pub is_time_limit: bool,
    pub is_infinite: bool,
}

impl Limits {
    fn elapsed(&self) -> u128 {
        self.start.elapsed().as_millis()
    }

    fn should_stop(&self) -> bool {
        self.is_time_limit && self.elapsed() >= self.spend
    }
}

fn printable_score(score: Score) -> (&'static str, Score) {
    if score.abs() >= MATE_IN_MAX {
        if score > 0 {
            ("mate", (MATE - score) / 2 + 1)
        } else {
            ("mate", -(MATE + score) / 2)
        }
    } else {
        ("cp", score)
    }
}

fn print_thinking(thread: &Thread, depth: u8, score: Score) {
    let elapsed = thread.limits.elapsed();
    let (score_type, score) = printable_score(score);
    let nodes = thread.get_global_nodes();
    let nps = (nodes as f64 * 1000.0 / (elapsed as f64 + 1.0)) as u64;
    let pv = thread.best_move.to_str(&thread.ci);
    println!(
        "info depth {} score {} {} time {} nodes {} nps {} pv {}",
        depth, score_type, score, elapsed, nodes, nps, pv
    );
}

pub fn start_search(thread: &mut Thread) {
    println!("info string static eval {}", eval(&thread.root));
    for d in 1..=thread.limits.depth {
        let pos = thread.root.clone();
        let score = search(thread, pos, d, 0, -INFINITE, INFINITE);
        if thread.id == 0 && !thread.abort.load(Ordering::Relaxed) {
            print_thinking(&thread, d, score);
        }
    }

    if thread.id == 0 {
        println!("bestmove {}", thread.best_move.to_str(&thread.ci));
    }
}

fn search(
    thread: &mut Thread,
    pos: Position,
    depth: u8,
    height: u8,
    mut alpha: Score,
    beta: Score,
) -> Score {
    thread.inc_nodes();
    let root = height == 0;
    let original_alpha = alpha;

    if thread.get_local_nodes() % CHECKUP_NODES == 0 && thread.limits.should_stop() {
        thread.abort.store(true, Ordering::Relaxed);
    }

    if thread.abort.load(Ordering::Relaxed) || !root && thread.hist.is_2fold(&pos) {
        return 0;
    }

    if depth == 0 {
        return eval(&pos);
    }

    let tt_entry = thread.tt().read(&pos);
    let mut tt_move = NO_MOVE;
    if let Some(tt_entry) = tt_entry {
        if !root
            && tt_entry.depth >= depth
            && (tt_entry.is_lower() && tt_entry.score >= beta
                || tt_entry.is_upper() && tt_entry.score <= alpha
                || tt_entry.is_exact())
        {
            return tt_entry.score;
        }
        tt_move = tt_entry.mv;
    }

    let mut move_count = 0;
    let mut best_score = -INFINITE;
    let mut best_move = NO_MOVE;

    for mv in pos.gen_pseudo_legals(&thread.ci) {
        let mut new_pos = pos.clone();
        if !new_pos.make_move(mv, &thread.ci) {
            continue;
        }
        thread.hist.push(&new_pos);

        move_count += 1;

        let score = -search(thread, new_pos, depth - 1, height + 1, -beta, -alpha);
        thread.hist.pop();

        if score > best_score {
            best_score = score;
            alpha = alpha.max(best_score);
            best_move = mv;
            if score >= beta {
                return score;
            }
        }
    }

    if move_count == 0 {
        return if pos.in_check(pos.ctm) {
            mate_score(height)
        } else {
            0
        };
    }

    if !thread.abort.load(Ordering::Relaxed) {
        if root {
            thread.best_move = best_move;
        }
        let flag = if best_score >= beta {
            FLAG_LOWER
        } else if best_score <= original_alpha {
            FLAG_UPPER
        } else {
            FLAG_EXACT
        };
        thread
            .tt()
            .insert(&pos, best_score, height, best_move, depth, flag);
    }

    best_score
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            start: Instant::now(),
            spend: 0,

            time: 0,
            inc: 0,

            movetime: 0,
            moves_to_go: 30,

            depth: MAX_DEPTH,
            mate: 0,

            is_time_limit: false,
            is_infinite: false,
        }
    }
}
