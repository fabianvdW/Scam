use crate::bitboard::*;
use crate::types::*;

#[derive(Default)]
pub struct Position {
    piece_bb: [BitBoard; 7],
    color_bb: [BitBoard; 2],

    pub ctm: Color,
    pub ep: Square,
    pub mr50: u8,
    pub cr: CastlingRights,

    pub fullmove: u8,
}

impl Position {
    pub fn color_bb(&self, color: Color) -> BitBoard {
        self.color_bb[color as usize]
    }

    pub fn piecetype_bb(&self, piece: PieceType) -> BitBoard {
        self.piece_bb[piece as usize]
    }

    pub fn piece_bb(&self, piece: Piece) -> BitBoard {
        self.piecetype_bb(piecetype_of(piece)) & self.color_bb(color_of(piece))
    }

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
        let piece = char_to_piece(piece_char);
        self.color_bb[color_of(piece) as usize] |= bb!(sq);
        self.piece_bb[piecetype_of(piece) as usize] |= bb!(sq);
        self.piece_bb[ALL as usize] |= bb!(sq);
    }

    pub fn startpos() -> Position {
        let startpos_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        Position::parse_fen(startpos_fen)
    }
}
