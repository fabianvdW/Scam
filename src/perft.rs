use crate::position::Position;
use crate::r#move::MoveList;

use std::time::Instant;

pub fn perft(args: Vec<&str>) {
    let depth: usize = args[1].parse().unwrap();
    let fen = args[2..].join(" ");
    let pos = Position::parse_fen(&fen);

    let start = Instant::now();
    let count = _perft(pos, depth);

    let time = start.elapsed().as_secs_f64();
    let nps = count as f64 / time;

    println!("\n{}", count);
    println!("Time {:.3} ({:.0} nps)", time, nps);
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
