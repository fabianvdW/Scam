use scam::position::*;
use std::io;
use std::time::Instant;
// use scam::*;

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    loop {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let arg: Vec<&str> = line.split_whitespace().collect();
        if line.is_empty() || arg.is_empty() {
            continue;
        }
        let cmd = arg[0];
        match cmd.trim() {
            "uci" => println!("uciok"),
            "isready" => println!("readyok"),
            "perft" => {
                let depth = arg[1].parse::<usize>().unwrap();
                let fen = arg[2..].join(" ").to_string();
                let pos = Position::parse_fen(&fen);
                let now = Instant::now();
                let count = scam::perft(pos, depth);
                let after = Instant::now();
                println!("{}", count);
                let dur = after.duration_since(now);
                let secs = dur.as_millis() as f64 / 1000.0;
                println!(
                    "{}",
                    &format!("Time {} ({} nps)", secs, count as f64 / secs)
                );
            }
            "quit" => {
                break;
            }
            _ => {}
        }
    }
}
