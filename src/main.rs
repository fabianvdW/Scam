use scam::position::{CastleInfo, Position};
use scam::r#move::Move;
use scam::types::*;
use scam::*;
use std::io::{prelude::*, stdin};

fn uci() {
    println!("id name Scam 0.0");
    println!("id author Fabian von der Warth, Terje Kirstihagen");
    println!("option name UCI_Chess960 type check default false");
    println!("uciok")
}

fn go(pos: &Position, ci: &CastleInfo, line: String) {
    let mut limits = search::Limits::default();
    let mut tokens = line.split_whitespace();
    let c = pos.ctm;

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

    search::start_search(&pos, ci.clone(), &limits);
}

fn position(pos: &mut Position, ci: &mut CastleInfo, line: String) {
    let (newpos, newci) = if line.contains("fen") {
        Position::parse_fen(line.splitn(3, ' ').nth(2).unwrap())
    } else {
        Position::startpos()
    };

    *pos = newpos;
    *ci = newci;

    if line.contains("moves ") {
        line.rsplit("moves ")
            .next()
            .unwrap()
            .split_whitespace()
            .for_each(|m| assert!(pos.make_move(Move::from_str(pos, ci, m), &ci)));
    }
}

fn setoption(line: String, ci: &mut CastleInfo) {
    let mut iter = line.rsplit("name ").next().unwrap().split(" value ");
    let name = iter.next().unwrap();
    let value = iter.next().unwrap();
    match name {
        "UCI_Chess960" => ci.frc = value.parse::<bool>().unwrap(),
        _ => println!("Unrecognized option: {}!", name),
    }
}

fn main() {
    if std::env::args().nth(1) == Some("bench".to_owned()) {
        return scam::bench::bench();
    }

    let (mut pos, mut ci) = Position::startpos();

    for line in stdin().lock().lines().map(|l| l.unwrap()) {
        let cmd = line.split_whitespace().next().unwrap_or("");
        match cmd {
            "go" => go(&pos, &ci, line),
            "uci" => uci(),
            "isready" => println!("readyok"),
            "setoption" => setoption(line, &mut ci),
            "position" => position(&mut pos, &mut ci, line),
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
