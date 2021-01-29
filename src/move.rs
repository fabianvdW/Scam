use crate::attacks::*;
use crate::bitboard::*;
use crate::position::{CastleInfo, Position};
use crate::types::*;

/* u16 Move construction
0000 0000 0011 1111 -> to square
0000 1111 1100 0000 -> from square
0011 0000 0000 0000 -> promotion type
1100 0000 0000 0000 -> move type
*/

pub type MoveType = u16;
pub const NORMAL: MoveType = 0;
pub const PROMOTION: MoveType = 1 << 14;
pub const ENPASSANT: MoveType = 2 << 14;
pub const CASTLING: MoveType = 3 << 14;
pub const NO_MOVE: Move = Move(0);

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Move(u16);

impl Move {
    pub fn new(from: Square, to: Square, mt: MoveType, promo: Option<PieceType>) -> Self {
        debug_assert!(mt == PROMOTION && promo.is_some() || mt != PROMOTION && promo.is_none());
        let p = promo.unwrap_or(KNIGHT);
        Move(mt | (((p - KNIGHT) as u16) << 12) | (from << 6) as u16 | to as u16)
    }

    pub const fn to(self) -> Square {
        (self.0 & 0x3F) as Square
    }

    pub const fn capture_to(self) -> Square {
        self.to() ^ (8 * (self.move_type() == ENPASSANT) as Square)
    }

    pub const fn from(self) -> Square {
        ((self.0 >> 6) & 0x3F) as Square
    }

    pub const fn move_type(self) -> MoveType {
        (self.0 & (3 << 14)) as MoveType
    }

    pub fn promo_type(self) -> PieceType {
        debug_assert_eq!(self.move_type(), PROMOTION);
        ((self.0 >> 12) & 3) as PieceType + KNIGHT
    }

    pub fn is_pseudolegal(&self, pos: &Position, ci: &CastleInfo) -> bool {
        let (from, to) = (self.from(), self.to());
        let (from_bb, to_bb) = (bb!(from), bb!(to));
        let color = pos.ctm;
        let occ = pos.piecetype_bb(ALL);

        if (from_bb & pos.color_bb(color)).is_empty() {
            return false;
        }

        let from_piece = pos.piece_on(from).unwrap();

        if piecetype_of(from_piece) == PAWN {
            return if self.move_type() == ENPASSANT {
                pos.ep == to
            } else {
                if (self.move_type() == PROMOTION) != (relative_rank(rank_of(to), color) == RANK_8)
                {
                    return false;
                }
                let enemies = pos.color_bb(swap_color(color));
                let push = pawn_push(from_bb, color, occ);
                let double = pawn_push(push & RANK_BB[relative_rank(RANK_3, color)], color, occ);
                ((push | double | (pawn_attack_bb(from, color) & enemies)) & to_bb).not_empty()
            };
        } else if self.move_type() == NORMAL {
            let targets = !pos.color_bb(color);
            return (attack_bb(piecetype_of(from_piece), from, occ) & to_bb & targets).not_empty();
        } else if piecetype_of(from_piece) == KING && self.move_type() == CASTLING {
            let queenside = from > to;
            let cr = [[W_KS, W_QS], [B_KS, B_QS]][color as usize][queenside as usize];
            if pos.cr & cr > 0
                && (ci.castle_path[cr as usize] & occ & !bb!(from, ci.castle_rooks[cr as usize]))
                    .is_empty()
            {
                return ci.castle_rooks[cr as usize] == to;
            }
        }

        false
    }

    pub fn from_str(pos: &Position, ci: &CastleInfo, s: &str) -> Move {
        for m in pos.gen_pseudo_legals(ci) {
            if m.to_str(ci) == s {
                return m;
            }
        }

        panic!("Invalid movestring given.")
    }

    pub fn to_str(self, ci: &CastleInfo) -> String {
        let from = square_to_str(self.from());
        let to = if self.move_type() == CASTLING && !ci.frc {
            let to = (bb!(self.to()).shift(WEST) | bb!(self.to()).shift(EAST + EAST)).lsb();
            square_to_str(to)
        } else {
            square_to_str(self.to())
        };
        let promo = if self.move_type() == PROMOTION {
            piecetype_to_char(self.promo_type()).to_string()
        } else {
            "".to_string()
        };
        format!("{}{}{}", from, to, promo)
    }
}

pub type ScoredMove = (Move, ());

pub struct MoveList {
    pub moves: [ScoredMove; 256],
    size: usize,
}

impl MoveList {
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        self.size = 0
    }

    pub fn pop(&mut self) -> Option<ScoredMove> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            Some(self.moves[self.size])
        }
    }

    pub fn push(&mut self, mv: Move) {
        self.moves[self.size] = (mv, ());
        self.size += 1;
    }
}

impl Default for MoveList {
    fn default() -> MoveList {
        let moves = unsafe { std::mem::MaybeUninit::uninit().assume_init() }; //UB
        MoveList { moves, size: 0 }
    }
}

impl Iterator for MoveList {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop().map(|x| x.0)
    }
}
