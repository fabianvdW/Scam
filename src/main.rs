use scam::*;
use std::io;

fn uci() {
    println!("id name Scam 0.0");
    println!("id author Fabian von der Warth, Terje Kirstihagen");
    println!("uciok")
}

fn main() {
    let stdin = io::stdin();
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let args: Vec<&str> = line.split_whitespace().collect();
        if line.is_empty() || args.is_empty() {
            continue;
        }
        let cmd = args[0].trim();
        match cmd {
            "uci" => uci(),
            "isready" => println!("readyok"),
            "perft" => perft::perft(args),
            "quit" => break,
            _ => {}
        }
    }
}
