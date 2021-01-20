use crate::position::Position;

use std::cmp::min;

#[derive(Clone)]
pub struct HashHist {
    hist: [u64; 256],
    pointer: usize,
}

impl Default for HashHist {
    fn default() -> Self {
        HashHist {
            hist: [0; 256],
            pointer: 0,
        }
    }
}

impl HashHist {
    pub fn clear(&mut self) {
        self.pointer = 0;
    }

    pub fn push(&mut self, pos: &Position) {
        self.hist[self.pointer] = pos.hash;
        self.pointer += 1;
    }

    pub fn pop(&mut self) {
        self.pointer -= 1;
    }

    pub fn is_2fold(&self, pos: &Position) -> bool {
        //hist[self.pointer -1] = pos.hash
        //hist[self.pointer -3] = pos_2movesago.hash != pos.hash
        //=> Start at 5, stride 2
        for i in (5..=min(self.pointer, 1 + pos.mr50 as usize)).step_by(2) {
            if self.hist[self.pointer - i] == pos.hash {
                return true;
            }
        }
        false
    }
}
