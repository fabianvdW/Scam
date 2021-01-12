use crate::attacks::*;
use crate::bitboard::*;
use crate::r#move::*;
use crate::transposition::hash;
use crate::types::*;
use std::borrow::BorrowMut;
use std::fmt;

#[derive(Clone)]
pub struct CastleInfo {
    pub castle_rights: [CastleRights; 64],
    pub castle_path: [BitBoard; 9],
    pub castle_rooks: [Square; 9],
    pub frc: bool,
}

#[derive(Copy, Clone, Default)]
pub struct Irreversible {
    mv: Move,
    captured_piece: Option<Piece>,
    ep: Square,
    mr50: u8,
    cr: CastleRights,
    fullmove: u8,
    hash: u64,
}

#[derive(Clone)]
pub struct Position {
    piece_bb: [BitBoard; 7],
    color_bb: [BitBoard; 2],
    board: [Piece; 64],

    pub ctm: Color,
    pub ep: Square,
    pub mr50: u8,
    pub cr: CastleRights,
    pub fullmove: u8,

    pub hash: u64,

    pub history: [Irreversible; 256],
    pub history_pointer: usize,
    pub ci: CastleInfo,
}

impl<'a> Position {
    pub fn piece_on(&self, sq: Square) -> Option<Piece> {
        match self.board[sq as usize] {
            0 => None,
            _ => Some(self.board[sq as usize]),
        }
    }

    pub fn unmake_move(&mut self) {
        self.history_pointer -= 1;
        let irr = self.history[self.history_pointer];
        self.ep = irr.ep;
        self.mr50 = irr.mr50;
        self.cr = irr.cr;
        self.fullmove = irr.fullmove;
        self.hash = irr.hash;
        let mv = irr.mv;
        let captured_piece = irr.captured_piece;
        self.ctm = swap_color(self.ctm);

        let (from, to) = (mv.from(), mv.to());

        if mv.move_type() == PROMOTION {
            self.toggle_piece_on_sq_nh(make_piece(self.ctm, PAWN), from);
            self.toggle_piece_on_sq_nh(make_piece(self.ctm, mv.promo_type()), to);
        } else if mv.move_type() == CASTLING {
            let c_types = [[W_KS, W_QS], [B_KS, B_QS]][self.ctm as usize];
            let c_type = if self.ci.castle_rooks[c_types[0] as usize] == to {
                c_types[0]
            } else {
                c_types[1]
            } as usize;
            self.move_piece_nh(make_piece(self.ctm, KING), from, CASTLE_K_TARGET[c_type]);
            self.move_piece_nh(make_piece(self.ctm, ROOK), to, CASTLE_R_TARGET[c_type]);
        } else {
            self.move_piece_nh(self.piece_on(to).unwrap(), from, to);
        }

        if mv.move_type() != CASTLING {
            if let Some(piece) = captured_piece {
                self.toggle_piece_on_sq_nh(piece, mv.capture_to());
            }
        }
    }

    pub fn make_move(&mut self, mv: Move) -> bool {
        self.history[self.history_pointer] = Irreversible::from((self.borrow_mut(), mv));
        self.history_pointer += 1;

        self.mr50 += 1;
        let (from, mut to) = (mv.from(), mv.to());
        let moving_piece = self.piece_on(from).unwrap(); // We have to initialize this here due to the fact that a friendly rook might temporarily move on top of our king on a FRC castle

        if mv.move_type() == CASTLING {
            if self.in_check(self.ctm) {
                self.history_pointer -= 1;
                self.mr50 -= 1;
                return false;
            }
            for &cr in [[W_KS, W_QS], [B_KS, B_QS]][self.ctm as usize].iter() {
                let r_from = self.ci.castle_rooks[cr as usize];
                if to == r_from {
                    let k_target = CASTLE_K_TARGET[cr as usize];
                    for sq in BETWEEN_BB[from as usize][k_target as usize] {
                        if self.square_attacked(sq, swap_color(self.ctm)) {
                            self.history_pointer -= 1;
                            self.mr50 -= 1;
                            return false;
                        }
                    }
                    let r_target = CASTLE_R_TARGET[cr as usize];
                    self.move_piece(make_piece(self.ctm, ROOK), r_from, r_target);
                    to = k_target;
                    break;
                }
            }
        } else if let Some(piece) = self.piece_on(mv.capture_to()) {
            debug_assert!(color_of(piece) != self.ctm);
            self.toggle_piece_on_sq(piece, mv.capture_to());
            self.mr50 = 0;
        }

        if mv.move_type() == PROMOTION {
            self.toggle_piece_on_sq(moving_piece, from);
            self.toggle_piece_on_sq(make_piece(self.ctm, mv.promo_type()), to);
        } else {
            self.move_piece(moving_piece, from, to);
        }

        self.fullmove += self.ctm;
        self.ctm = swap_color(self.ctm);
        // Can't be in check after we removed the enemy piece and moved our piece
        if self.in_check(swap_color(self.ctm)) {
            self.unmake_move();
            return false;
        }

        // Default EP-Square is A1 which has hash key 0 so xoring it in/out has no effect
        self.hash ^= hash::EP[self.ep as usize];
        self.ep = A1;
        if piecetype_of(moving_piece) == PAWN {
            self.mr50 = 0;
            if to ^ from == 16 {
                self.ep = ep_captured_sq(to);
                self.hash ^= hash::EP[self.ep as usize];
            }
        }

        self.hash ^= hash::CASTLE_RIGHTS[self.cr as usize];
        self.cr &= self.ci.castle_rights[from as usize] & self.ci.castle_rights[to as usize];
        self.hash ^= hash::CASTLE_RIGHTS[self.cr as usize];

        self.hash ^= hash::CTM;
        true
    }

    fn move_piece(&mut self, piece: Piece, from_sq: Square, to_sq: Square) {
        self.toggle_piece_on_sq(piece, from_sq);
        self.toggle_piece_on_sq(piece, to_sq);
    }

    fn toggle_piece_on_sq(&mut self, piece: Piece, sq: Square) {
        self.board[sq as usize] ^= piece;
        self.piece_bb[piecetype_of(piece) as usize] ^= bb!(sq);
        self.color_bb[color_of(piece) as usize] ^= bb!(sq);
        self.piece_bb[ALL as usize] ^= bb!(sq);
        self.hash ^= hash::PIECES[piece as usize][sq as usize];
    }

    fn move_piece_nh(&mut self, piece: Piece, from_sq: Square, to_sq: Square) {
        self.toggle_piece_on_sq_nh(piece, from_sq);
        self.toggle_piece_on_sq_nh(piece, to_sq);
    }

    fn toggle_piece_on_sq_nh(&mut self, piece: Piece, sq: Square) {
        self.board[sq as usize] ^= piece;
        self.piece_bb[piecetype_of(piece) as usize] ^= bb!(sq);
        self.color_bb[color_of(piece) as usize] ^= bb!(sq);
        self.piece_bb[ALL as usize] ^= bb!(sq);
    }

    pub fn square_attacked(&self, sq: Square, c: Color) -> bool {
        let (bishops, rooks) = (self.bishop_likes_bb(c), self.rook_likes_bb(c));
        (attack_bb(KNIGHT, sq, BB_ZERO) & self.piece_bb(KNIGHT, c)).not_empty()
            || (attack_bb(BISHOP, sq, self.piecetype_bb(ALL)) & bishops).not_empty()
            || (attack_bb(ROOK, sq, self.piecetype_bb(ALL)) & rooks).not_empty()
            || (pawn_attack_bb(sq, swap_color(c)) & self.piece_bb(PAWN, c)).not_empty()
            || (attack_bb(KING, sq, BB_ZERO) & self.piece_bb(KING, c)).not_empty()
    }

    pub fn in_check(&self, c: Color) -> bool {
        self.square_attacked(self.king_sq(c), swap_color(c))
    }

    pub fn gen_pseudo_legals(&self) -> MoveList {
        let mut list = MoveList::default();

        let color = self.ctm;
        let occ = self.piecetype_bb(ALL);
        let targets = !self.color_bb(color);
        let enemies = self.color_bb(swap_color(color));

        let our_piece = |x| self.piece_bb(x, color);

        // Non-pawns
        for &pt in [KING, KNIGHT, BISHOP, ROOK, QUEEN].iter() {
            for from in our_piece(pt) {
                let attacks = attack_bb(pt, from, occ) & targets;
                for to in attacks {
                    list.push(Move::new(from, to, NORMAL, None));
                }
            }
        }

        // Pawns
        let pawns_on7th = our_piece(PAWN) & RANK_BB[relative_rank(RANK_7, color)];
        let pawns_not7th = our_piece(PAWN) ^ pawns_on7th;

        let push = pawn_push(pawns_not7th, color, occ);
        let double = pawn_push(push & RANK_BB[relative_rank(RANK_3, color)], color, occ);
        let west_attacks = pawn_bb_west_bb(pawns_not7th, color);
        let east_attacks = pawn_bb_east_bb(pawns_not7th, color);

        macro_rules! pawn_pseudolegals {($ ($dir: expr, $mt: ident, $targets: expr); +) => {$(
            for to in $targets{
                let from = (to as Direction - relative_dir($dir, color)) as Square;
                list.push(Move::new(from, to, $mt, None));
            }
        )+};}
        pawn_pseudolegals!(
            NORTH, NORMAL, push;
            NORTH+NORTH, NORMAL, double;
            NORTH_WEST, NORMAL, west_attacks & enemies;
            NORTH_EAST, NORMAL, east_attacks & enemies;
            NORTH_WEST, ENPASSANT, west_attacks & bb!(self.ep);
            NORTH_EAST, ENPASSANT, east_attacks & bb!(self.ep)
        );

        macro_rules! pawn_promos {($ ($dir: expr, $targets: expr); +) => {$(
            for to in $targets {
                let from = (to as Direction - relative_dir($dir, color)) as Square;
                for &promo in [KNIGHT, BISHOP, ROOK, QUEEN].iter() {
                    list.push(Move::new(from, to, PROMOTION, Some(promo)))
                }
            }
        )+};}
        pawn_promos!(
            NORTH, pawn_push(pawns_on7th, color, occ);
            NORTH_WEST, pawn_bb_west_bb(pawns_on7th, color) & enemies;
            NORTH_EAST, pawn_bb_east_bb(pawns_on7th, color) & enemies
        );

        // Castling
        let k_sq = self.king_sq(color);
        for &cr in [[W_KS, W_QS], [B_KS, B_QS]][color as usize].iter() {
            if (self.cr & cr) > 0
                && (self.ci.castle_path[cr as usize]
                    & occ
                    & !bb!(k_sq, self.ci.castle_rooks[cr as usize]))
                .is_empty()
            {
                let k_target = self.ci.castle_rooks[cr as usize];
                list.push(Move::new(k_sq, k_target, CASTLING, None));
            }
        }

        list
    }

    pub fn color_bb(&self, c: Color) -> BitBoard {
        self.color_bb[c as usize]
    }

    pub fn piecetype_bb(&self, pt: PieceType) -> BitBoard {
        self.piece_bb[pt as usize]
    }

    pub fn piece_bb(&self, pt: PieceType, c: Color) -> BitBoard {
        self.piecetype_bb(pt) & self.color_bb(c)
    }

    pub fn king_sq(&self, c: Color) -> Square {
        (self.piecetype_bb(KING) & self.color_bb(c)).lsb()
    }

    pub fn bishop_likes_bb(&self, c: Color) -> BitBoard {
        (self.piecetype_bb(BISHOP) | self.piecetype_bb(QUEEN)) & self.color_bb(c)
    }

    pub fn rook_likes_bb(&self, c: Color) -> BitBoard {
        (self.piecetype_bb(ROOK) | self.piecetype_bb(QUEEN)) & self.color_bb(c)
    }

    pub fn reset_history(&mut self) {
        self.history_pointer = 0;
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
            "b" => {
                pos.ctm = BLACK;
                pos.hash ^= hash::CTM;
            }
            _ => panic!("Invalid color in FEN."),
        }

        pos.ci.castle_rights = [ALL_CASTLING; 64];
        let (w_rooks, b_rooks) = (pos.piece_bb(ROOK, WHITE), pos.piece_bb(ROOK, BLACK));
        for c in tokens.next().unwrap().chars() {
            match c {
                'K' => pos.init_castle(WHITE, file_of((w_rooks & RANK_1_BB).msb())),
                'Q' => pos.init_castle(WHITE, file_of((w_rooks & RANK_1_BB).lsb())),
                'k' => pos.init_castle(BLACK, file_of((b_rooks & RANK_8_BB).msb())),
                'q' => pos.init_castle(BLACK, file_of((b_rooks & RANK_8_BB).lsb())),
                'a'..='h' | 'A'..='H' => {
                    pos.ci.frc = true; //Note that this does not cover all cases of FRC we could detect
                    let color = c.is_ascii_lowercase() as Color;
                    let file = char_to_file(c.to_ascii_lowercase());
                    pos.init_castle(color, file)
                }
                '-' => break,
                _ => panic!("Invalid castling rights in FEN."),
            };
        }

        match tokens.next() {
            Some("-") => (),
            Some(ep) => {
                pos.ep = str_to_square(ep);
                pos.hash ^= hash::EP[pos.ep as usize]
            }
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

    fn init_castle(&mut self, color: Color, file: File) {
        let king_sq = self.king_sq(color);
        let king_file = file_of(king_sq);
        let rook_sq = to_square([RANK_1, RANK_8][color as usize], file);
        let cr = [[W_KS, B_KS], [W_QS, B_QS]][(file < king_file) as usize][color as usize];
        self.hash ^= hash::CASTLE_RIGHTS[self.cr as usize];
        self.cr |= cr;
        self.hash ^= hash::CASTLE_RIGHTS[self.cr as usize];
        self.ci.castle_rooks[cr as usize] = rook_sq;
        self.ci.castle_rights[rook_sq as usize] &= !cr;
        self.ci.castle_rights[king_sq as usize] &= !cr;
        self.ci.castle_path[cr as usize] = between_inc_bb(king_sq, CASTLE_K_TARGET[cr as usize])
            | between_inc_bb(rook_sq, CASTLE_R_TARGET[cr as usize]);
    }

    fn add_piece(&mut self, piece_char: char, sq: Square) {
        let piece = char_to_piece(piece_char);
        self.toggle_piece_on_sq(piece, sq);
    }

    pub fn startpos() -> Position {
        let startpos_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        Position::parse_fen(startpos_fen)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::default();
        for rank in (RANK_1..RANK_NB).rev() {
            for file in FILE_A..FILE_NB {
                res.push(piece_to_char(
                    self.piece_on(to_square(rank, file)).unwrap_or(0),
                ));
            }
            res.push('\n')
        }
        res.push_str(&format!("Hash: {}", self.hash));
        f.write_str(&res)
    }
}

impl From<(&mut Position, Move)> for Irreversible {
    fn from((pos, mv): (&mut Position, Move)) -> Self {
        Irreversible {
            mv,
            captured_piece: pos.piece_on(mv.capture_to()), //Careful: If mv.move_type == CASTLING, captured_piece = Some(ROOK)
            ep: pos.ep,
            mr50: pos.mr50,
            cr: pos.cr,
            fullmove: pos.fullmove,
            hash: pos.hash,
        }
    }
}

//Be gone once Default is implemented with const generics
impl Default for Position {
    fn default() -> Position {
        Position {
            piece_bb: [BB_ZERO; 7],
            color_bb: [BB_ZERO; 2],
            board: [0; 64],

            ctm: 0,
            ep: 0,
            mr50: 0,
            cr: 0,
            fullmove: 0,

            hash: 0,
            history: [Irreversible::default(); 256],
            history_pointer: 0,
            ci: CastleInfo::default(),
        }
    }
}

impl Default for CastleInfo {
    fn default() -> CastleInfo {
        CastleInfo {
            castle_rights: [0; 64],
            castle_path: [BB_ZERO; 9],
            castle_rooks: [A2; 9],
            frc: false,
        }
    }
}
