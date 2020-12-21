use crate::attacks::*;
use crate::bitboard::*;
use crate::r#move::*;
use crate::types::*;

#[derive(Clone)]
pub struct Position {
    piece_bb: [BitBoard; 7],
    color_bb: [BitBoard; 2],

    pub ctm: Color,
    pub ep: Square,
    pub mr50: u8,
    pub cr: CastlingRights,
    pub castle_permissions: [CastlingRights; 64],
    pub castle_path: [BitBoard; 9],
    pub castle_rooks: [Square; 9],

    pub fullmove: u8,
}

impl Default for Position {
    fn default() -> Position {
        Position {
            piece_bb: [BB_ZERO; 7],
            color_bb: [BB_ZERO; 2],
            ctm: WHITE,
            ep: A1,
            mr50: 0,
            cr: 0,
            castle_permissions: [0; 64],
            castle_path: [BB_ZERO; 9],
            castle_rooks: [A1; 9],
            fullmove: 0,
        }
    }
}

impl Position {
    pub fn piece_on(&self, sq: Square) -> Option<Piece> {
        for &pt in [PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING].iter() {
            if (self.piece_bb[pt as usize] & bb!(sq)).not_empty() {
                return Some(make_piece(
                    (self.color_bb[BLACK as usize] & bb!(sq)).not_empty() as Color,
                    pt,
                ));
            }
        }
        None
    }

    pub fn make_move(&mut self, mv: Move) -> bool {
        if mv.move_type() == CASTLING {
            if self.in_check(self.ctm) {
                return false;
            }
            for &cr in [[W_KS, W_QS], [B_KS, B_QS]][self.ctm as usize].iter() {
                let k_target = CASTLE_K_TARGET[cr as usize];
                if mv.to() == k_target {
                    for sq in BETWEEN_BB[self.king_sq(self.ctm) as usize][k_target as usize] {
                        if self.square_attacked(sq, swap_color(self.ctm)) {
                            return false;
                        }
                    }
                }
            }
        }

        self.mr50 += 1;

        if let Some(piece) = self.piece_on(mv.capture_to()) {
            debug_assert!(color_of(piece) != self.ctm || mv.move_type() == CASTLING);
            if color_of(piece) != self.ctm {
                self.toggle_piece_on_sq(piece, mv.capture_to());
                self.mr50 = 0;
            }
        }

        let moving_piece = self.piece_on(mv.from()).unwrap();
        if mv.move_type() == PROMOTION {
            self.toggle_piece_on_sq(moving_piece, mv.from());
            self.toggle_piece_on_sq(make_piece(self.ctm, mv.promo_type()), mv.to());
        } else {
            self.move_piece(moving_piece, mv.from(), mv.to());
        }

        if mv.move_type() == CASTLING {
            for &cr in [[W_KS, W_QS], [B_KS, B_QS]][self.ctm as usize].iter() {
                let k_target = CASTLE_K_TARGET[cr as usize];
                let r_target = CASTLE_R_TARGET[cr as usize];
                if mv.to() == k_target {
                    let r_from = self.castle_rooks[cr as usize];
                    self.move_piece(make_piece(self.ctm, ROOK), r_from, r_target);
                    break;
                }
            }
        }
        // Can't be in check after we removed the enemy piece and moved our piece
        if self.in_check(self.ctm) {
            return false;
        }

        self.ep = A1;
        if piecetype_of(moving_piece) == PAWN {
            self.mr50 = 0;
            if (mv.to() as i32 - mv.from() as i32).abs() == 16 {
                self.ep = ep_captured_sq(mv.to());
            }
        }

        self.cr &=
            self.castle_permissions[mv.from() as usize] & self.castle_permissions[mv.to() as usize];
        self.fullmove += self.ctm;
        self.ctm = swap_color(self.ctm);
        true
    }

    fn move_piece(&mut self, piece: Piece, from_sq: Square, to_sq: Square) {
        self.toggle_piece_on_sq(piece, from_sq);
        self.toggle_piece_on_sq(piece, to_sq);
    }

    fn toggle_piece_on_sq(&mut self, piece: Piece, sq: Square) {
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

    pub fn gen_pseudo_legals(&self, list: &mut MoveList) {
        list.clear();

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

        let pushes = (NORTH, NORMAL, push);
        let doubles = (NORTH + NORTH, NORMAL, double);
        let wests = (NORTH_WEST, NORMAL, west_attacks & enemies);
        let easts = (NORTH_EAST, NORMAL, east_attacks & enemies);
        let ep_wests = (NORTH_WEST, ENPASSANT, west_attacks & bb!(self.ep));
        let ep_easts = (NORTH_EAST, ENPASSANT, east_attacks & bb!(self.ep));

        for (dir, mt, targets) in [pushes, doubles, wests, easts, ep_wests, ep_easts].iter() {
            for to in *targets {
                let from = (to as Direction - relative_dir(*dir, color)) as Square;
                list.push(Move::new(from, to, *mt, None))
            }
        }

        // Promotions
        let pushes = (NORTH, pawn_push(pawns_on7th, color, occ));
        let wests = (NORTH_WEST, pawn_bb_west_bb(pawns_on7th, color) & enemies);
        let easts = (NORTH_EAST, pawn_bb_east_bb(pawns_on7th, color) & enemies);

        for (dir, targets) in [pushes, wests, easts].iter() {
            for to in *targets {
                let from = (to as Direction - relative_dir(*dir, color)) as Square;
                for &promo in [KNIGHT, BISHOP, ROOK, QUEEN].iter() {
                    list.push(Move::new(from, to, PROMOTION, Some(promo)))
                }
            }
        }

        // Castling
        let k_sq = self.king_sq(color);
        for &cr in [[W_KS, W_QS], [B_KS, B_QS]][color as usize].iter() {
            if (self.cr & cr) > 0
                && (self.castle_path[cr as usize]
                    & occ
                    & !bb!(k_sq, self.castle_rooks[cr as usize]))
                .is_empty()
            {
                let k_target = CASTLE_K_TARGET[cr as usize];
                list.push(Move::new(k_sq, k_target, CASTLING, None));
            }
        }
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

        pos.castle_permissions = [15; 64];
        let king_squares = [pos.king_sq(WHITE), pos.king_sq(BLACK)];
        pos.castle_permissions[king_squares[WHITE as usize] as usize] = B_KS | B_QS;
        pos.castle_permissions[king_squares[BLACK as usize] as usize] = W_KS | W_QS;
        pos.castle_path[W_KS as usize] = between_inc_bb(king_squares[WHITE as usize], G1);
        pos.castle_path[W_QS as usize] = between_inc_bb(king_squares[WHITE as usize], C1);
        pos.castle_path[B_KS as usize] = between_inc_bb(king_squares[BLACK as usize], G8);
        pos.castle_path[B_QS as usize] = between_inc_bb(king_squares[BLACK as usize], C8);

        let w_rooks = pos.piece_bb(ROOK, WHITE);
        let b_rooks = pos.piece_bb(ROOK, BLACK);
        let mut setup_rook = |cr: CastlingRights, rook_sq: Square| {
            pos.cr |= cr;
            pos.castle_rooks[cr as usize] = rook_sq;
            pos.castle_permissions[rook_sq as usize] &= !cr;
            pos.castle_path[cr as usize] |= between_inc_bb(rook_sq, CASTLE_R_TARGET[cr as usize]);
        };
        for c in tokens.next().unwrap().chars() {
            match c {
                'K' => setup_rook(W_KS, (w_rooks & RANK_1_BB).hsb()),
                'Q' => setup_rook(W_QS, (w_rooks & RANK_1_BB).lsb()),
                'k' => setup_rook(B_KS, (b_rooks & RANK_8_BB).hsb()),
                'q' => setup_rook(B_QS, (b_rooks & RANK_8_BB).lsb()),
                'a'..='h' | 'A'..='H' => {
                    let file = char_to_file(c.to_ascii_lowercase());
                    let color = c.is_ascii_lowercase() as usize;
                    let king_file = file_of(king_squares[color] as Square);
                    let cr = [[W_KS, B_KS], [W_QS, B_QS]][(file < king_file) as usize][color];
                    setup_rook(cr, to_square([RANK_1, RANK_8][color], file))
                }
                '-' => break,
                _ => panic!("Invalid castling rights in FEN."),
            };
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
