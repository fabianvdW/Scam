use crate::attacks::*;
use crate::bitboard::*;
use crate::r#move::*;
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
    pub fn gen_pseudo_legals(&self, list: &mut MoveList) {
        list.clear();

        let our_piece = |x| make_piece(self.ctm, x);
        let our_pieces = self.color_bb(self.ctm);
        let enemy_pieces = self.color_bb(swap_color(self.ctm));
        let all_pieces = self.piecetype_bb(ALL);

        for &pt in [KING, KNIGHT, BISHOP, ROOK, QUEEN].iter() {
            for from_sq in self.piece_bb(our_piece(pt)) {
                let attacks = attack_bb(pt, from_sq, all_pieces) & !our_pieces;
                for to_sq in attacks {
                    list.push(Move::new(from_sq, to_sq, NORMAL, None));
                }
            }
        }

        let pawns_on7th = self.piece_bb(our_piece(PAWN)) & RANK_BB[relative_rank(RANK_7, self.ctm)];
        let pawns_not7th = self.piece_bb(our_piece(PAWN)) ^ pawns_on7th;

        let _pushes = pawn_bb_pushes_bb(self.ctm, pawns_not7th, all_pieces);
        let pushes = (NORTH, NORMAL, _pushes.0);
        let d_pushes = (NORTH + NORTH, NORMAL, _pushes.1);
        let pawn_west_attacks = pawn_bb_west_bb(self.ctm, pawns_not7th);
        let pawn_east_attacks = pawn_bb_east_bb(self.ctm, pawns_not7th);
        let wests = (NORTH_WEST, NORMAL, pawn_west_attacks & enemy_pieces);
        let easts = (NORTH_EAST, NORMAL, pawn_east_attacks & enemy_pieces);
        let ep_wests = (NORTH_WEST, ENPASSANT, pawn_west_attacks & bb!(self.ep));
        let ep_easts = (NORTH_EAST, ENPASSANT, pawn_east_attacks & bb!(self.ep));
        for (dir, mt, targets) in [pushes, d_pushes, wests, easts, ep_wests, ep_easts].iter() {
            for to in *targets {
                let from = (to as Direction - relative_dir(*dir, self.ctm)) as Square;
                list.push(Move::new(from, to, *mt, None))
            }
        }

        let pushes = (NORTH, pawn_bb_singles_bb(self.ctm, pawns_on7th, all_pieces));
        let wests = (
            NORTH_WEST,
            pawn_bb_west_bb(self.ctm, pawns_on7th) & enemy_pieces,
        );
        let easts = (
            NORTH_EAST,
            pawn_bb_east_bb(self.ctm, pawns_on7th) & enemy_pieces,
        );
        for (dir, targets) in [pushes, wests, easts].iter() {
            for to in *targets {
                let from = (to as Direction - relative_dir(*dir, self.ctm)) as Square;
                for &promo in [KNIGHT, BISHOP, ROOK, QUEEN].iter() {
                    list.push(Move::new(from, to, PROMOTION, Some(promo)))
                }
            }
        }

        if self.ctm == WHITE {
            if (self.cr & W_KS > 0) && (bb!(F1, G1) & all_pieces == BB_ZERO) {
                list.push(Move::new(E1, G1, CASTLING, None));
            }
            if (self.cr & W_QS > 0) && (bb!(D1, C1, B1) & all_pieces == BB_ZERO) {
                list.push(Move::new(E1, C1, CASTLING, None));
            }
        } else {
            if (self.cr & B_KS > 0) && (bb!(F8, G8) & all_pieces == BB_ZERO) {
                list.push(Move::new(E8, G8, CASTLING, None));
            }
            if (self.cr & B_QS > 0) && (bb!(D8, C8, B8) & all_pieces == BB_ZERO) {
                list.push(Move::new(E8, C8, CASTLING, None));
            }
        }
    }

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
