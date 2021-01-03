use scam::position::{CastleInfo, Position};
use scam::r#move::Move;
use scam::types::ALL;
use scam::*;
use std::io::{prelude::*, stdin};

fn uci() {
    println!("id name Scam 0.0");
    println!("id author Fabian von der Warth, Terje Kirstihagen");
    println!("uciok")
}

fn position(pos: &mut Position, ci: &mut CastleInfo, line: String) {
    let mut tokens = line.splitn(3, ' ');
    tokens.next();
    let option = tokens.next().unwrap();
    let rest = tokens.next();

    let (newpos, newci) = if option == "fen" {
        Position::parse_fen(rest.unwrap())
    } else {
        Position::startpos()
    };

    *pos = newpos;
    *ci = newci;

    if line.contains("moves ") {
        let moves = rest.unwrap().rsplit("moves ").next().unwrap();
        moves
            .split_whitespace()
            .for_each(|m| assert!(pos.make_move(Move::from_str(pos, m), &ci)));
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
            "uci" => uci(),
            "isready" => println!("readyok"),
            "position" => position(&mut pos, &mut ci, line),
            "print" => print!("{:b}", pos.piecetype_bb(ALL)),
            "perft" => perft::perft(line),
            "quit" => break,
            "bench" => scam::bench::bench(),
            _ => {}
        }
    }
}
