use crate::position::Position;
use crate::types::*;

const PIECE_VALUES: [i32; 6] = [0, 100, 325, 350, 550, 1000];

pub fn eval(pos: &Position) -> i32 {
    let mut eval = 0;

    for &pt in [PAWN, KNIGHT, BISHOP, ROOK, QUEEN].iter() {
        eval += pos.piece_bb(pt, WHITE).popcount() as i32 * PIECE_VALUES[pt as usize];
        eval -= pos.piece_bb(pt, BLACK).popcount() as i32 * PIECE_VALUES[pt as usize];
    }

    eval
}
