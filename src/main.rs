use scam::*;
use std::io::{prelude::*, stdin};

fn uci() {
    println!("id name Scam 0.0");
    println!("id author Fabian von der Warth, Terje Kirstihagen");
    println!("uciok")
}

fn main() {
    for line in stdin().lock().lines().map(|l| l.unwrap()) {
        let args: Vec<&str> = line.split_whitespace().collect();
        if args.is_empty() {
            continue;
        }
        match args[0] {
            "uci" => uci(),
            "isready" => println!("readyok"),
            "perft" => perft::perft(args),
            "quit" => break,
            _ => {}
        }
    }
}
