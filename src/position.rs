use crate::bitboard::*;
use crate::squares::*;

pub type Color = u8;
pub const WHITE: Color = 0;
pub const BLACK: Color = 1;

pub fn char_to_rank(c: char) -> u8 {
    assert!("12345678".contains(c));
    c as u8 - b'1'
}

pub fn char_to_file(c: char) -> u8 {
    assert!("abcdefgh".contains(c));
    c as u8 - b'a'
}

pub fn str_to_square(s: &str) -> Square {
    let file = char_to_file(s.chars().next().unwrap());
    let rank = char_to_rank(s.chars().nth(1).unwrap());
    (file + rank * 8) as Square
}

fn parse_piece(piece_char: char) -> u8 {
    let char_to_piece = ".PNBRQK..pnbrqk";
    char_to_piece.find(piece_char).unwrap() as u8
}

pub fn color_of(piece: u8) -> Color {
    piece >> 3
}

pub fn piecetype_of(piece: u8) -> u8 {
    piece & 7
}

pub fn rank_of(sq: Square) -> usize {
    (sq >> 3) as usize
}

pub fn file_of(sq: Square) -> usize {
    (sq & 7) as usize
}

#[derive(Default)]
pub struct Position {
    piece_bb: [BitBoard; 7],
    color_bb: [BitBoard; 2],

    ctm: Color,
    ep: Square,
    mr50: u8,
    cr: u8,

    fullmove: u8,
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
                'K' => pos.cr |= 1,
                'Q' => pos.cr |= 2,
                'k' => pos.cr |= 4,
                'q' => pos.cr |= 8,
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
