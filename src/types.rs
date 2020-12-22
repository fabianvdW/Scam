use crate::bitboard::*;

// Direction
pub type Direction = i8;

pub const NORTH: Direction = 8;
pub const SOUTH: Direction = -8;
pub const EAST: Direction = 1;
pub const WEST: Direction = -1;

pub const NORTH_EAST: Direction = 9;
pub const NORTH_WEST: Direction = 7;
pub const SOUTH_EAST: Direction = -7;
pub const SOUTH_WEST: Direction = -9;

pub const ROOK_DIRS: [Direction; 4] = [NORTH, SOUTH, WEST, EAST];
pub const BISHOP_DIRS: [Direction; 4] = [NORTH_WEST, NORTH_EAST, SOUTH_WEST, SOUTH_EAST];
pub const KING_DIRS: [Direction; 8] = [
    NORTH_WEST, NORTH, NORTH_EAST, EAST, WEST, SOUTH_WEST, SOUTH, SOUTH_EAST,
];
pub const KNIGHT_DIRS: [Direction; 8] = [
    NORTH + NORTH_WEST,
    NORTH + NORTH_EAST,
    SOUTH + SOUTH_WEST,
    SOUTH + SOUTH_EAST,
    EAST + NORTH_EAST,
    EAST + SOUTH_EAST,
    WEST + NORTH_WEST,
    WEST + SOUTH_WEST,
];

pub const fn relative_dir(dir: Direction, color: Color) -> Direction {
    if color == WHITE {
        dir
    } else {
        -dir
    }
}

// Color
pub type Color = u8;

pub const WHITE: Color = 0;
pub const BLACK: Color = 1;

pub const fn swap_color(c: Color) -> Color {
    c ^ 1
}

// PieceType
pub type PieceType = u8;

pub const ALL: PieceType = 0;
pub const PAWN: PieceType = 1;
pub const KNIGHT: PieceType = 2;
pub const BISHOP: PieceType = 3;
pub const ROOK: PieceType = 4;
pub const QUEEN: PieceType = 5;
pub const KING: PieceType = 6;

pub fn piecetype_to_char(pt: PieceType) -> char {
    ".pnbrqk".chars().nth(pt as usize).unwrap()
}

// Piece
pub type Piece = u8;

pub const W_PAWN: Piece = 1;
pub const W_KNIGHT: Piece = 2;
pub const W_BISHOP: Piece = 3;
pub const W_ROOK: Piece = 4;
pub const W_QUEEN: Piece = 5;
pub const W_KING: Piece = 6;

pub const B_PAWN: Piece = 9;
pub const B_KNIGHT: Piece = 10;
pub const B_BISHOP: Piece = 11;
pub const B_ROOK: Piece = 12;
pub const B_QUEEN: Piece = 13;
pub const B_KING: Piece = 14;

pub const fn make_piece(color: Color, pt: PieceType) -> Piece {
    (color << 3) + pt
}

pub const fn piecetype_of(piece: Piece) -> PieceType {
    piece & 7
}

pub const fn color_of(piece: Piece) -> Color {
    piece >> 3
}

pub fn char_to_piece(piece_char: char) -> Piece {
    let char_to_piece = ".PNBRQK..pnbrqk";
    char_to_piece.find(piece_char).unwrap() as Piece
}

// Rank
pub type Rank = usize;

pub const RANK_NB: usize = 8;

pub const RANK_1: Rank = 0;
pub const RANK_2: Rank = 1;
pub const RANK_3: Rank = 2;
pub const RANK_4: Rank = 3;
pub const RANK_5: Rank = 4;
pub const RANK_6: Rank = 5;
pub const RANK_7: Rank = 6;
pub const RANK_8: Rank = 7;

pub const RANK_1_BB: BitBoard = bb!(A1, B1, C1, D1, E1, F1, G1, H1);
pub const RANK_2_BB: BitBoard = bb!(A2, B2, C2, D2, E2, F2, G2, H2);
pub const RANK_3_BB: BitBoard = bb!(A3, B3, C3, D3, E3, F3, G3, H3);
pub const RANK_4_BB: BitBoard = bb!(A4, B4, C4, D4, E4, F4, G4, H4);
pub const RANK_5_BB: BitBoard = bb!(A5, B5, C5, D5, E5, F5, G5, H5);
pub const RANK_6_BB: BitBoard = bb!(A6, B6, C6, D6, E6, F6, G6, H6);
pub const RANK_7_BB: BitBoard = bb!(A7, B7, C7, D7, E7, F7, G7, H7);
pub const RANK_8_BB: BitBoard = bb!(A8, B8, C8, D8, E8, F8, G8, H8);

pub const RANK_BB: [BitBoard; 8] = [
    RANK_1_BB, RANK_2_BB, RANK_3_BB, RANK_4_BB, RANK_5_BB, RANK_6_BB, RANK_7_BB, RANK_8_BB,
];

pub const fn relative_rank(rank: Rank, c: Color) -> Rank {
    if c == WHITE {
        rank
    } else {
        RANK_8 - rank
    }
}

pub const fn rank_of(sq: Square) -> Rank {
    (sq >> 3) as Rank
}

pub fn char_to_rank(c: char) -> Rank {
    debug_assert!(('1'..='8').contains(&c));
    (c as u8 - b'1') as Rank
}

pub fn rank_to_char(rank: Rank) -> char {
    debug_assert!((RANK_1..=RANK_8).contains(&rank));
    (rank as u8 + b'1') as char
}

// File
pub type File = usize;

pub const FILE_NB: usize = 8;

pub const FILE_A: File = 0;
pub const FILE_B: File = 1;
pub const FILE_C: File = 2;
pub const FILE_D: File = 3;
pub const FILE_E: File = 4;
pub const FILE_F: File = 5;
pub const FILE_G: File = 6;
pub const FILE_H: File = 7;

pub const FILE_A_BB: BitBoard = bb!(A1, A2, A3, A4, A5, A6, A7, A8);
pub const FILE_B_BB: BitBoard = bb!(B1, B2, B3, B4, B5, B6, B7, B8);
pub const FILE_C_BB: BitBoard = bb!(C1, C2, C3, C4, C5, C6, C7, C8);
pub const FILE_D_BB: BitBoard = bb!(D1, D2, D3, D4, D5, D6, D7, D8);
pub const FILE_E_BB: BitBoard = bb!(E1, E2, E3, E4, E5, E6, E7, E8);
pub const FILE_F_BB: BitBoard = bb!(F1, F2, F3, F4, F5, F6, F7, F8);
pub const FILE_G_BB: BitBoard = bb!(G1, G2, G3, G4, G5, G6, G7, G8);
pub const FILE_H_BB: BitBoard = bb!(H1, H2, H3, H4, H5, H6, H7, H8);

pub const FILE_BB: [BitBoard; 8] = [
    FILE_A_BB, FILE_B_BB, FILE_C_BB, FILE_D_BB, FILE_E_BB, FILE_F_BB, FILE_G_BB, FILE_H_BB,
];

pub const fn file_of(sq: Square) -> File {
    (sq & 7) as File
}

pub fn char_to_file(c: char) -> File {
    debug_assert!(('a'..='h').contains(&c));
    (c as u8 - b'a') as File
}

pub fn file_to_char(file: File) -> char {
    debug_assert!((FILE_A..=FILE_H).contains(&file));
    (file as u8 + b'a') as char
}

// Square
pub type Square = u32;

pub const SQUARE_NB: usize = 64;

pub const A1: Square = 0;
pub const B1: Square = 1;
pub const C1: Square = 2;
pub const D1: Square = 3;
pub const E1: Square = 4;
pub const F1: Square = 5;
pub const G1: Square = 6;
pub const H1: Square = 7;
pub const A2: Square = 8;
pub const B2: Square = 9;
pub const C2: Square = 10;
pub const D2: Square = 11;
pub const E2: Square = 12;
pub const F2: Square = 13;
pub const G2: Square = 14;
pub const H2: Square = 15;
pub const A3: Square = 16;
pub const B3: Square = 17;
pub const C3: Square = 18;
pub const D3: Square = 19;
pub const E3: Square = 20;
pub const F3: Square = 21;
pub const G3: Square = 22;
pub const H3: Square = 23;
pub const A4: Square = 24;
pub const B4: Square = 25;
pub const C4: Square = 26;
pub const D4: Square = 27;
pub const E4: Square = 28;
pub const F4: Square = 29;
pub const G4: Square = 30;
pub const H4: Square = 31;
pub const A5: Square = 32;
pub const B5: Square = 33;
pub const C5: Square = 34;
pub const D5: Square = 35;
pub const E5: Square = 36;
pub const F5: Square = 37;
pub const G5: Square = 38;
pub const H5: Square = 39;
pub const A6: Square = 40;
pub const B6: Square = 41;
pub const C6: Square = 42;
pub const D6: Square = 43;
pub const E6: Square = 44;
pub const F6: Square = 45;
pub const G6: Square = 46;
pub const H6: Square = 47;
pub const A7: Square = 48;
pub const B7: Square = 49;
pub const C7: Square = 50;
pub const D7: Square = 51;
pub const E7: Square = 52;
pub const F7: Square = 53;
pub const G7: Square = 54;
pub const H7: Square = 55;
pub const A8: Square = 56;
pub const B8: Square = 57;
pub const C8: Square = 58;
pub const D8: Square = 59;
pub const E8: Square = 60;
pub const F8: Square = 61;
pub const G8: Square = 62;
pub const H8: Square = 63;

pub const fn ep_captured_sq(ep_target_sq: Square) -> Square {
    ep_target_sq ^ 8
}

pub const fn to_square(rank: Rank, file: File) -> Square {
    (8 * rank + file) as Square
}

pub fn str_to_square(s: &str) -> Square {
    let file = char_to_file(s.chars().next().unwrap());
    let rank = char_to_rank(s.chars().nth(1).unwrap());
    (file + rank * 8) as Square
}

pub fn square_to_str(sq: Square) -> String {
    let file_c = file_to_char(file_of(sq));
    let rank_c = rank_to_char(rank_of(sq));
    format!("{}{}", file_c, rank_c)
}

const DISTANCE: [[u8; 64]; 64] = {
    let mut res = [[0; 64]; 64];

    let mut sq1 = A1;
    while sq1 <= H8 {
        let mut sq2 = A1;
        while sq2 <= H8 {
            let vert = (rank_of(sq1) as i32 - rank_of(sq2) as i32).abs();
            let hori = (file_of(sq1) as i32 - file_of(sq2) as i32).abs();
            res[sq1 as usize][sq2 as usize] = [vert, hori][(vert < hori) as usize] as u8;
            sq2 += 1;
        }
        sq1 += 1;
    }

    res
};

pub const fn distance(sq1: Square, sq2: Square) -> u8 {
    DISTANCE[sq1 as usize][sq2 as usize]
}

// CastleRights
pub type CastleRights = u8;

pub const W_KS: CastleRights = 1;
pub const W_QS: CastleRights = 2;
pub const B_KS: CastleRights = 4;
pub const B_QS: CastleRights = 8;

pub const CASTLE_K_TARGET: [Square; 9] = [A1, G1, C1, A1, G8, A1, A1, A1, C8];
pub const CASTLE_R_TARGET: [Square; 9] = [A1, F1, D1, A1, F8, A1, A1, A1, D8];
