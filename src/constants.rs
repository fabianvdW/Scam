use crate::bitboard::*;
use crate::squares::*;

pub const FILE_A: BitBoard = bb!(A1, A2, A3, A4, A5, A6, A7, A8);
pub const FILE_B: BitBoard = bb!(B1, B2, B3, B4, B5, B6, B7, B8);
pub const FILE_C: BitBoard = bb!(C1, C2, C3, C4, C5, C6, C7, C8);
pub const FILE_D: BitBoard = bb!(D1, D2, D3, D4, D5, D6, D7, D8);
pub const FILE_E: BitBoard = bb!(E1, E2, E3, E4, E5, E6, E7, E8);
pub const FILE_F: BitBoard = bb!(F1, F2, F3, F4, F5, F6, F7, F8);
pub const FILE_G: BitBoard = bb!(G1, G2, G3, G4, G5, G6, G7, G8);
pub const FILE_H: BitBoard = bb!(H1, H2, H3, H4, H5, H6, H7, H8);
pub const FILES: [BitBoard; 8] = [
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H,
];
pub const MAX_FILES: usize = 8;

pub const RANK_1: BitBoard = bb!(A1, B1, C1, D1, E1, F1, G1, H1);
pub const RANK_2: BitBoard = bb!(A2, B2, C2, D2, E2, F2, G2, H2);
pub const RANK_3: BitBoard = bb!(A3, B3, C3, D3, E3, F3, G3, H3);
pub const RANK_4: BitBoard = bb!(A4, B4, C4, D4, E4, F4, G4, H4);
pub const RANK_5: BitBoard = bb!(A5, B5, C5, D5, E5, F5, G5, H5);
pub const RANK_6: BitBoard = bb!(A6, B6, C6, D6, E6, F6, G6, H6);
pub const RANK_7: BitBoard = bb!(A7, B7, C7, D7, E7, F7, G7, H7);
pub const RANK_8: BitBoard = bb!(A8, B8, C8, D8, E8, F8, G8, H8);
pub const RANKS: [BitBoard; 8] = [
    RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
];
pub const MAX_RANKS: usize = 8;
