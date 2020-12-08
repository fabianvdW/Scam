use crate::bitboard::*;
use crate::types::*;

include!(concat!(env!("OUT_DIR"), "/magic_attacks.rs"));
impl Magic {
    #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
    pub fn bishop_attacks(sq: Square, occ: BitBoard) -> BitBoard {
        BitBoard(ATTACKS[BISHOP_MAGICS[sq as usize].apply_bmi2(occ)])
    }

    #[cfg(not(all(target_arch = "x86_64", target_feature = "bmi2")))]
    pub fn bishop_attacks(sq: Square, occ: BitBoard) -> BitBoard {
        BitBoard(ATTACKS[BISHOP_MAGICS[sq as usize].apply_magic(occ)])
    }

    #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
    pub fn rook_attacks(sq: Square, occ: BitBoard) -> BitBoard {
        BitBoard(ATTACKS[ROOK_MAGICS[sq as usize].apply_bmi2(occ)])
    }

    #[cfg(not(all(target_arch = "x86_64", target_feature = "bmi2")))]
    pub fn rook_attacks(sq: Square, occ: BitBoard) -> BitBoard {
        BitBoard(ATTACKS[ROOK_MAGICS[sq as usize].apply_magic(occ)])
    }
}
