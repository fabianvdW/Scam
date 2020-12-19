use crate::types::*;
use std::fmt::Display;

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

#[derive(Clone, Copy, PartialEq, Eq)]
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
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            square_to_str(self.from()),
            square_to_str(self.to()),
            if self.move_type() == PROMOTION {
                piecetype_to_char(self.promo_type()).to_string()
            } else {
                String::new()
            }
        )
    }
}

pub type ScoredMove = (Move, ());

pub struct MoveList {
    moves: [ScoredMove; 256],
    size: usize,
}

impl MoveList {
    pub fn clear(&mut self) {
        self.size = 0
    }

    pub fn pop(&mut self) -> Option<ScoredMove> {
        if self.size == 0 {
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
