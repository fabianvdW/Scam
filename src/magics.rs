use crate::bitboard::*;
use crate::types::*;

include!(concat!(env!("OUT_DIR"), "/magic_attacks.rs"));
impl Magic {
    pub fn bishop_attacks(sq: Square, occ: BitBoard) -> BitBoard {
        BitBoard(ATTACKS[BISHOP_MAGICS[sq as usize].index(occ)])
    }

    pub fn rook_attacks(sq: Square, occ: BitBoard) -> BitBoard {
        BitBoard(ATTACKS[ROOK_MAGICS[sq as usize].index(occ)])
    }
}
