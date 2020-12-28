use scam::*;
use std::io::{prelude::*, stdin};

fn uci() {
    println!("id name Scam 0.0");
    println!("id author Fabian von der Warth, Terje Kirstihagen");
    println!("uciok")
}

fn main() {
    for line in stdin().lock().lines().map(|l| l.unwrap()) {
        let cmd = line.split_whitespace().next().unwrap_or("");
        match cmd {
            "uci" => uci(),
            "isready" => println!("readyok"),
            "perft" => perft::perft(line),
            "quit" => break,
            _ => {}
        }
    }
}
