use crate::bitboard::*;
use crate::magics::*;
use crate::types::*;

include!(concat!(env!("OUT_DIR"), "/magic_attacks.rs"));

const KING_ATTACKS: [BitBoard; 64] = init_non_slider_attacks(KING_DIRS);
const KNIGHT_ATTACKS: [BitBoard; 64] = init_non_slider_attacks(KNIGHT_DIRS);

pub const fn pawn_attack_bb(sq: Square, c: Color) -> BitBoard {
    pawn_bb_attack_bb(bb!(sq), c)
}

pub const fn pawn_bb_west_bb(pawns: BitBoard, c: Color) -> BitBoard {
    pawns.shift(relative_dir(NORTH_WEST, c))
}

pub const fn pawn_bb_east_bb(pawns: BitBoard, c: Color) -> BitBoard {
    pawns.shift(relative_dir(NORTH_EAST, c))
}

pub const fn pawn_bb_attack_bb(pawns: BitBoard, c: Color) -> BitBoard {
    pawn_bb_west_bb(pawns, c).or(pawn_bb_east_bb(pawns, c))
}

pub const fn pawn_push(pawns: BitBoard, c: Color, occ: BitBoard) -> BitBoard {
    let up = relative_dir(NORTH, c);
    pawns.shift(up).and(not!(occ))
}

pub fn attack_bb(pt: PieceType, sq: Square, occ: BitBoard) -> BitBoard {
    match pt {
        KNIGHT => knight_attacks(sq),
        BISHOP => bishop_attacks(sq, occ),
        ROOK => rook_attacks(sq, occ),
        QUEEN => bishop_attacks(sq, occ) | rook_attacks(sq, occ),
        KING => king_attacks(sq),
        _ => panic!("Bad piecetype for attack_bb()."),
    }
}

fn bishop_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    BitBoard(ATTACKS[BISHOP_MAGICS[sq as usize].index(occ)])
}

fn rook_attacks(sq: Square, occ: BitBoard) -> BitBoard {
    BitBoard(ATTACKS[ROOK_MAGICS[sq as usize].index(occ)])
}

const fn knight_attacks(sq: Square) -> BitBoard {
    KNIGHT_ATTACKS[sq as usize]
}

const fn king_attacks(sq: Square) -> BitBoard {
    KING_ATTACKS[sq as usize]
}

const fn init_non_slider_attacks(dirs: [Direction; 8]) -> [BitBoard; 64] {
    let mut res = [BitBoard(0); 64];

    let mut sq = 0;
    while sq < SQUARE_NB {
        let mut dir = 0;
        while dir < 8 {
            let to = (sq as i8 + dirs[dir]) as usize;
            if to < SQUARE_NB && distance(sq as Square, to as Square) <= 2 {
                res[sq] = res[sq].or(bb!(to));
            }
            dir += 1;
        }
        sq += 1;
    }

    res
}
