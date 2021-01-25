use crate::position::{CastleInfo, Position};
use crate::r#move::*;
pub const STAGE_TT: usize = 0;
pub const STAGE_GEN: usize = 1;
pub const STAGE_NORMAL: usize = 2;

pub struct MovePicker<'a> {
    pos: &'a Position,
    tt_move: Move,
    pub stage: usize,
    pub movelist: MoveList,
}
impl<'a> MovePicker<'a> {
    pub fn new(pos: &'a Position, tt_move: Move) -> MovePicker<'a> {
        MovePicker {
            pos,
            tt_move,
            stage: 0,
            movelist: MoveList::default(),
        }
    }

    pub fn next(&mut self, ci: &CastleInfo) -> Option<Move> {
        match self.stage {
            STAGE_TT => {
                self.stage += 1;
                if self.tt_move != NO_MOVE && self.pos.is_valid_pseudolegal(self.tt_move, ci) {
                    Some(self.tt_move)
                } else {
                    self.next(ci)
                }
            }
            STAGE_GEN => {
                self.pos._gen_pseudo_legals(ci, &mut self.movelist);
                self.stage += 1;
                self.next(ci)
            }
            STAGE_NORMAL => {
                let res = self.movelist.next();
                if res != Some(self.tt_move) {
                    res
                } else {
                    self.next(ci)
                }
            }
            _ => None,
        }
    }
}
