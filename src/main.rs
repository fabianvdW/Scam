use scam::history::HashHist;
use scam::position::{CastleInfo, Position};
use scam::r#move::Move;
use scam::thread::SharedState;
use scam::types::*;
use scam::*;

use std::io::{prelude::*, stdin};
use std::sync::atomic::Ordering;

fn uci() {
    println!("id name Scam 0.4");
    println!("id author Fabian von der Warth, Terje Kirstihagen");
    println!("option name UCI_Chess960 type check default false");
    println!("option name Threads type spin default 1 min 1 max 65536");
    println!("option name Hash type spin default 2 min 1 max 2147483647");
    println!("uciok")
}

fn go(
    pos: &Position,
    ci: &CastleInfo,
    hist: &HashHist,
    shared_state: &mut SharedState,
    line: String,
) {
    let mut limits = search::Limits::default();
    let mut tokens = line.split_whitespace();
    let c = pos.ctm;
    let overhead = 5;

    macro_rules! value {
        () => {
            tokens.next().unwrap().parse().unwrap()
        };
    }

    while let Some(content) = tokens.next() {
        match content {
            "infinite" => limits.is_infinite = true,
            "wtime" if c == WHITE => limits.time = value!(),
            "btime" if c == BLACK => limits.time = value!(),
            "winc" if c == WHITE => limits.inc = value!(),
            "binc" if c == BLACK => limits.inc = value!(),
            "movestogo" => limits.moves_to_go = value!(),
            "movetime" => limits.movetime = value!(),
            "depth" => limits.depth = value!(),
            "mate" => limits.mate = value!(),
            _ => {}
        }
    }

    limits.is_time_limit = limits.time != 0 || limits.movetime != 0;

    // Basic time management for now
    limits.spend = if limits.movetime > 0 {
        limits.movetime.saturating_sub(overhead)
    } else {
        (limits.time / limits.moves_to_go + limits.inc).min(limits.time.saturating_sub(overhead))
    };
    shared_state.start_search(pos.clone(), ci.clone(), hist.clone(), limits);
}

fn position(pos: &mut Position, ci: &mut CastleInfo, hist: &mut HashHist, line: String) {
    let (newpos, newci) = if line.contains("fen") {
        Position::parse_fen(line.splitn(3, ' ').nth(2).unwrap())
    } else {
        Position::startpos()
    };

    *pos = newpos;
    *ci = newci;
    hist.clear();
    hist.push(pos);

    if line.contains("moves ") {
        line.rsplit("moves ")
            .next()
            .unwrap()
            .split_whitespace()
            .for_each(|m| {
                assert!(pos.make_move(Move::from_str(pos, ci, m), &ci));
                if pos.mr50 == 0 {
                    hist.clear();
                }
                hist.push(pos);
            });
    }
}

fn setoption(line: String, ci: &mut CastleInfo, shared_state: &mut SharedState) {
    let mut iter = line.rsplit("name ").next().unwrap().split(" value ");
    let name = iter.next().unwrap();
    let value = iter.next().unwrap();
    match name {
        "UCI_Chess960" => ci.frc = value.parse().unwrap(),
        "Threads" => shared_state.launch_threads(value.parse().unwrap()),
        "Hash" => shared_state.reallocate_tt(value.parse().unwrap()),
        _ => println!("Unrecognized option: {}!", name),
    }
}

fn main() {
    if std::env::args().nth(1) == Some("bench".to_owned()) {
        return scam::bench::bench();
    } else if std::env::args().nth(1) == Some("perftbench".to_owned()) {
        return scam::bench::perftbench();
    }

    let (mut pos, mut ci) = Position::startpos();
    let mut hist = HashHist::default();
    let mut shared_state = SharedState::default();
    shared_state.launch_threads(1);

    for line in stdin().lock().lines().map(|l| l.unwrap()) {
        let cmd = line.split_whitespace().next().unwrap_or("");
        match cmd {
            "go" => go(&pos, &ci, &hist, &mut shared_state, line),
            "uci" => uci(),
            "isready" => println!("readyok"),
            "setoption" => setoption(line, &mut ci, &mut shared_state),
            "position" => position(&mut pos, &mut ci, &mut hist, line),
            "stop" => shared_state.abort.store(true, Ordering::Relaxed),
            "quit" => break,
            // Non-UCI commands
            "eval" => println!("{}", eval::eval(&pos)),
            "perft" => perft::perft(line),
            "bench" => scam::bench::bench(),
            "print" => println!("{}", pos),
            _ => {}
        }
    }
}
