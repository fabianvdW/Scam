use crate::bitboard::*;
use crate::types::*;

include!(concat!(env!("OUT_DIR"), "/magic_attacks.rs"));
impl Magic {
    pub fn bishop_attacks(sq: Square, occ: BitBoard) -> BitBoard {
        if cfg!(all(target_arch = "x64_64", target_feature = "bmi2")) {
            BitBoard(ATTACKS[BISHOP_MAGICS[sq as usize].apply_magic(occ)])
        } else {
            BitBoard(ATTACKS[BISHOP_MAGICS[sq as usize].apply_bmi2(occ)])
        }
    }

    pub fn rook_attacks(sq: Square, occ: BitBoard) -> BitBoard {
        if cfg!(all(target_arch = "x64_64", target_feature = "bmi2")) {
            BitBoard(ATTACKS[ROOK_MAGICS[sq as usize].apply_bmi2(occ)])
        } else {
            BitBoard(ATTACKS[ROOK_MAGICS[sq as usize].apply_magic(occ)])
        }
    }
}
