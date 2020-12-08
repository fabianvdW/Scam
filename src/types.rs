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

pub const fn relative_dir(dir: Direction, color: Color) {
    if color == WHITE {
        dir
    } else {
        -dir
    };
}

// Color
pub type Color = u8;

pub const WHITE: Color = 0;
pub const BLACK: Color = 1;

// PieceType
pub type PieceType = u8;

pub const ALL: PieceType = 0;
pub const PAWN: PieceType = 1;
pub const KNIGHT: PieceType = 2;
pub const BISHOP: PieceType = 3;
pub const ROOK: PieceType = 4;
pub const QUEEN: PieceType = 5;
pub const KING: PieceType = 6;

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

pub fn parse_piece(piece_char: char) -> Piece {
    let char_to_piece = ".PNBRQK..pnbrqk";
    char_to_piece.find(piece_char).unwrap() as Piece
}

// CastlingRights
pub type CastlingRights = u8;

pub const W_KS: CastlingRights = 1;
pub const W_QS: CastlingRights = 2;
pub const B_KS: CastlingRights = 4;
pub const B_QS: CastlingRights = 8;

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

pub const fn rank_of(sq: Square) -> Rank {
    (sq >> 3) as Rank
}

pub fn char_to_rank(c: char) -> u8 {
    assert!("12345678".contains(c));
    c as u8 - b'1'
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

pub fn char_to_file(c: char) -> u8 {
    assert!("abcdefgh".contains(c));
    c as u8 - b'a'
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

pub fn str_to_square(s: &str) -> Square {
    let file = char_to_file(s.chars().next().unwrap());
    let rank = char_to_rank(s.chars().nth(1).unwrap());
    (file + rank * 8) as Square
}

// Magics
#[derive(Clone, Copy)]
pub struct Magic {
    pub mask: BitBoard,
    pub magic: u64,
    pub offset: usize,
    pub shift: u32,
}

impl Magic {
    pub const fn default() -> Magic {
        Magic {
            mask: BB_ZERO,
            magic: 0,
            offset: 0,
            shift: 0,
        }
    }

    pub fn index(&self, occ: BitBoard) -> usize {
        #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
        {
            use std::arch::x86_64::_pext_u64;
            self.offset + unsafe { _pext_u64(occ.0, self.mask.0) } as usize
        }
        #[cfg(not(all(target_arch = "x86_64", target_feature = "bmi2")))]
        {
            self.offset + (((occ & self.mask).0).wrapping_mul(self.magic) >> self.shift) as usize
        }
    }
}

pub const fn occupancy_mask(sq: Square, attack_dirs: [Direction; 4]) -> BitBoard {
    let mut res = BB_ZERO;
    let edges = (RANK_1_BB.or(RANK_8_BB).and(not!(RANK_BB[rank_of(sq)])))
        .or((FILE_A_BB.or(FILE_H_BB)).and(not!(FILE_BB[file_of(sq)])));

    let mut i = 0;
    while i < 4 {
        let dir = attack_dirs[i];
        let mut temp = bb!(sq).shift(dir);
        let mut j = 0;
        while j < 5 {
            temp = temp.or(temp.shift(dir));
            j += 1;
        }
        res = res.or(temp);
        i += 1;
    }
    res.and(not!(edges))
}

pub const fn init_magics(
    magic_nums: [u64; 64],
    attack_dirs: [Direction; 4],
    mut offset: usize,
) -> [Magic; 64] {
    let mut magics = [Magic::default(); 64];
    let mut sq = 0;
    while sq < SQUARE_NB {
        magics[sq].magic = magic_nums[sq];
        magics[sq].mask = occupancy_mask(sq as Square, attack_dirs);
        magics[sq].shift = 64 - magics[sq].mask.popcount();
        magics[sq].offset = offset;
        offset += 1 << 64 - magics[sq].shift;
        sq += 1;
    }
    magics
}

// Setting up Bishop magics
#[rustfmt::skip]
pub const BISHOP_MAGIC_NUMBERS : [u64; 64] = [9052302183530624u64, 3493106745918750722u64, 10378547575765598208u64, 1737267960881348624u64, 10173093901303832u64, 4666011823880819232u64, 595602155869570050u64, 4611897984056627264u64, 36249008850862404u64, 2216337449216u64, 2305851882628841472u64, 184651999957483520u64, 7494011856613818624u64, 1197984168606171392u64, 2256765064877074u64, 147774504575173632u64, 9232379519711904000u64, 1589780154182344962u64, 5843420671266299912u64, 2306970043015012930u64, 291610284032786432u64, 1412881035952660u64, 18577349571281920u64, 288265571328395280u64, 20398418977359873u64, 4616194017980600448u64, 2308105804345245712u64, 4611826893489045536u64, 9009398294841476u64, 2634606881531924610u64, 283674285703424u64, 1261300437177876736u64, 19333830213640194u64, 9225705209122014272u64, 36314674337548288u64, 5188148971919900801u64, 16289522094180425736u64, 81082939529527360u64, 5198622926656012808u64, 9656916352225543296u64, 2261180160746545u64, 40818338457190912u64, 1152932510729241088u64, 148919646538486784u64, 10134203572167168u64, 1135797138883072u64, 164383759939144704u64, 9233225930963681536u64, 100207325126067208u64, 1153207386539033088u64, 4611361466745472u64, 57139560060289058u64, 288248037091186432u64, 1301584865623408704u64, 75611525158570019u64, 146384586526490896u64, 1164255287713071617u64, 288338171259344900u64, 5764607534879117377u64, 1157495747864957184u64, 3222077704u64, 4616752605052544032u64, 2343072610411356416u64, 73218686973968530u64, ];
pub const BISHOP_MAGICS: [Magic; 64] = init_bishop_magics();

pub const fn init_bishop_magics() -> [Magic; 64] {
    init_magics(BISHOP_MAGIC_NUMBERS, BISHOP_DIRS, 0)
}

// Setting up Rook magics
#[rustfmt::skip]
pub const ROOK_MAGIC_NUMBERS : [u64; 64] = [2630106718943609138u64, 18032010559799296u64, 180161586023891074u64, 2449967268337156128u64, 36037593179127810u64, 1297037861652529664u64, 216173881668150784u64, 144115755014179329u64, 9516246750663278729u64, 2392674749399056u64, 14777779876790404u64, 578853461412548608u64, 36169551687712896u64, 4925820690762752u64, 422225358362112u64, 10387834016590004226u64, 468374636126535876u64, 2305918051150733312u64, 1153062792119508996u64, 40532946536465424u64, 5770519597325746180u64, 9223662312756613184u64, 36103566096597521u64, 9228176902740050052u64, 1242995973202911360u64, 301811597467189376u64, 3103015342663795328u64, 5944769102463107204u64, 5764629515414798465u64, 3458766714999669760u64, 288232592363292688u64, 290483284066992324u64, 351855003566724u64, 1371381339630076098u64, 2307021687834566656u64, 576496040862028288u64, 2955521640369152u64, 24910690066104832u64, 149602367980033u64, 140738620818688u64, 140738562129952u64, 4620836158493032480u64, 1157636347922546704u64, 4802950260195336u64, 8800388317200u64, 297959129979814176u64, 9017713502715912u64, 360429292935315457u64, 2306267730627658240u64, 666534181534443776u64, 360596933493932288u64, 288250168435319296u64, 7036908795364608u64, 2307531895849878016u64, 864708755556017152u64, 11608168789920731776u64, 144255964230459459u64, 4719808153548554754u64, 36117037123772417u64, 4756118072021484801u64, 581245895669196801u64, 563037070164226u64, 4684025104663969825u64, 2256199512819778u64, ];
pub const ROOK_MAGICS: [Magic; 64] = init_rook_magics();

pub const fn init_rook_magics() -> [Magic; 64] {
    init_magics(ROOK_MAGIC_NUMBERS, ROOK_DIRS, 5248)
}
