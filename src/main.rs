use scam::position::*;
use std::time::Instant;
// use scam::*;

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let pos = Position::parse_fen(fen);
    let now = Instant::now();
    let count = scam::perft(pos, 6);
    let after = Instant::now();
    println!("{}", count);
    let dur = after.duration_since(now);
    let secs = dur.as_millis() as f64 / 1000.0;
    println!(
        "{}",
        &format!("Time {} ({} nps)", secs, count as f64 / secs)
    );
}
