use crate::bitboard::*;
use crate::types::*;

pub type Color = u8;
pub const WHITE: Color = 0;
pub const BLACK: Color = 1;

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
fn parse_piece(piece_char: char) -> Piece {
    let char_to_piece = ".PNBRQK..pnbrqk";
    char_to_piece.find(piece_char).unwrap() as Piece
}
pub const fn color_of(piece: Piece) -> Color {
    piece >> 3
}

pub const fn piecetype_of(piece: Piece) -> Piece {
    piece & 7
}

pub type PieceType = u8;
pub const ALL: PieceType = 0;
pub const PAWN: PieceType = 1;
pub const KNIGHT: PieceType = 2;
pub const BISHOP: PieceType = 3;
pub const ROOK: PieceType = 4;
pub const QUEEN: PieceType = 5;
pub const KING: PieceType = 6;

pub type CastlingRights = u8;
pub const W_KS: CastlingRights = 1;
pub const W_QS: CastlingRights = 2;
pub const B_KS: CastlingRights = 4;
pub const B_QS: CastlingRights = 8;

#[derive(Default)]
pub struct Position {
    pub piece_bb: [BitBoard; 7],
    pub color_bb: [BitBoard; 2],

    pub ctm: Color,
    pub ep: Square,
    pub mr50: u8,
    pub cr: CastlingRights,

    pub fullmove: u8,
}

impl Position {
    pub fn parse_fen(fen: &str) -> Position {
        let mut pos = Position::default();
        let mut tokens = fen.split_ascii_whitespace();

        let mut sq = A8;
        for c in tokens.next().unwrap().chars() {
            match c {
                '/' => sq -= 16,
                '1'..='8' => sq += c.to_digit(10).unwrap(),
                _ => {
                    pos.add_piece(c, sq);
                    sq += 1;
                }
            }
        }

        match tokens.next().unwrap() {
            "w" => pos.ctm = WHITE,
            "b" => pos.ctm = BLACK,
            _ => panic!("Invalid color in FEN."),
        }

        for c in tokens.next().unwrap().chars() {
            match c {
                'K' => pos.cr |= W_KS,
                'Q' => pos.cr |= W_QS,
                'k' => pos.cr |= B_KS,
                'q' => pos.cr |= B_QS,
                _ => panic!("Invalid castling rights in FEN."),
            }
        }

        match tokens.next() {
            Some("-") => (),
            Some(ep) => pos.ep = str_to_square(ep),
            _ => panic!("Invalid en passant in FEN."),
        }

        pos.mr50 = tokens
            .next()
            .unwrap_or("0")
            .parse()
            .expect("Invalid halfmove counter in FEN.");

        pos.fullmove = tokens
            .next()
            .unwrap_or("1")
            .parse()
            .expect("Invalid fullmove counter in FEN.");

        pos
    }

    fn add_piece(&mut self, piece_char: char, sq: Square) {
        let piece = parse_piece(piece_char);
        self.color_bb[color_of(piece) as usize] |= bb!(sq);
        self.piece_bb[piecetype_of(piece) as usize] |= bb!(sq);
    }

    pub fn startpos() -> Position {
        let startpos_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        Position::parse_fen(startpos_fen)
    }
}
