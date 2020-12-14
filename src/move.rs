use crate::types::*;

/* u16 Move construction
0000 0000 0011 1111 -> to square
0000 1111 1100 0000 -> from square
0011 0000 0000 0000 -> promotion type
1100 0000 0000 0000 -> move type
*/

pub const MAX_MOVES: usize = 256;
pub type MoveType = u16;
pub const NORMAL: MoveType = 0;
pub const PROMOTION: MoveType = 1 << 14;
pub const ENPASSANT: MoveType = 2 << 14;
pub const CASTLING: MoveType = 3 << 14;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Move(u16);

pub type ScoredMove = (Move, ());

pub struct MoveList {
    moves: [ScoredMove; MAX_MOVES],
    size: usize,
}

impl Move {
    pub fn new(to: Square, from: Square, mt: MoveType, promo: Option<PieceType>) -> Self {
        let p = promo.unwrap_or(KNIGHT);
        Move(mt | (((p - KNIGHT) as u16) << 12) | (from << 6) as u16 | to as u16)
    }

    pub fn to(self) -> Square {
        (self.0 & 0x3F) as Square
    }

    pub fn from(self) -> Square {
        ((self.0 >> 6) & 0x3F) as Square
    }

    pub fn move_type(self) -> MoveType {
        (self.0 & (3 << 14)) as MoveType
    }

    pub fn promo_type(self) -> PieceType {
        debug_assert_eq!(self.move_type(), PROMOTION);
        ((self.0 >> 12) & 3) as PieceType + KNIGHT
    }
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
