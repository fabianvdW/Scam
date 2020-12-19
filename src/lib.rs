use crate::position::Position;

#[macro_use]
pub mod bitboard;
pub mod attacks;
pub mod magics;
pub mod r#move;
pub mod position;
pub mod types;

pub fn perft(pos: Position, depth: usize) -> u64 {
    if depth == 0 {
        return 1;
    }
    let mut res = 0;
    let mut mv_list = r#move::MoveList::default();
    pos.gen_pseudo_legals(&mut mv_list);
    for mv in mv_list {
        let mut new_pos = pos.clone();
        if new_pos.make_move(mv) {
            res += perft(new_pos, depth - 1);
        }
    }
    res
}
