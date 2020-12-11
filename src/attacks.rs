use crate::bitboard::*;
use crate::magics::*;
use crate::types::*;

include!(concat!(env!("OUT_DIR"), "/magic_attacks.rs"));

const KING_ATTACKS: [BitBoard; 64] = init_non_slider_attacks(KING_DIRS);
const KNIGHT_ATTACKS: [BitBoard; 64] = init_non_slider_attacks(KNIGHT_DIRS);

pub fn pawn_attack_bb(c: Color, sq: Square) -> BitBoard {
    pawn_bb_attack_bb(c, bb!(sq))
}

pub fn pawn_bb_attack_bb(c: Color, pawns: BitBoard) -> BitBoard {
    let up = if c == WHITE { NORTH } else { SOUTH };
    pawns.shift(up + WEST) | pawns.shift(up + EAST)
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

fn knight_attacks(sq: Square) -> BitBoard {
    KNIGHT_ATTACKS[sq as usize]
}

fn king_attacks(sq: Square) -> BitBoard {
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
                res[sq] = res[sq].or(BitBoard(1 << to));
            }
            dir += 1;
        }
        sq += 1;
    }

    res
}
