use crate::position::Position;
use crate::r#move::MoveList;

use std::time::Instant;

const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";

pub fn perft(line: String) {
    let mut tokens = line.split_whitespace();
    let depth: usize = tokens.nth(1).unwrap_or("5").parse().unwrap();
    let mut fen: &str = &tokens.collect::<Vec<&str>>().join(" ");
    if fen == "" {
        fen = KIWIPETE
    };

    let pos = Position::parse_fen(&fen);

    let start = Instant::now();
    let count = _perft(pos, depth);

    let time = start.elapsed().as_secs_f64();
    let nps = count as f64 / time;

    println!("\n{}", count);
    println!("Time {:.3} ({:.0} nps)\n", time, nps);
}

fn _perft(pos: Position, depth: usize) -> u64 {
    if depth == 0 {
        return 1;
    }
    let mut res = 0;
    let mut mv_list = MoveList::default();
    pos.gen_pseudo_legals(&mut mv_list);
    for mv in mv_list {
        let mut new_pos = pos.clone();
        if new_pos.make_move(mv) {
            res += _perft(new_pos, depth - 1);
        }
    }
    res
}
