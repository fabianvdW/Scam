use crate::position::Position;

use std::time::Instant;

const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

pub fn perft(line: String) {
    let mut tokens = line.split_whitespace();
    let depth: usize = tokens.nth(1).unwrap_or("5").parse().unwrap();
    let mut fen: &str = &tokens.collect::<Vec<&str>>().join(" ");
    if fen.is_empty() {
        fen = KIWIPETE
    };

    let mut pos = Position::parse_fen(&fen);

    let start = Instant::now();
    let count = _perft(&mut pos, depth);

    let time = start.elapsed().as_secs_f64();
    let nps = count as f64 / time;

    println!("\n{}", count);
    println!("Time {:.3} ({:.0} nps)\n", time, nps);
}

pub fn _perft(pos: &mut Position, depth: usize) -> u64 {
    if depth == 0 {
        return 1;
    }
    let mut res = 0;
    for mv in pos.gen_pseudo_legals() {
        if pos.make_move(mv) {
            res += _perft(pos, depth - 1);
            pos.unmake_move();
        }
    }
    res
}
