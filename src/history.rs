use crate::position::Position;

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
        for i in 1..=pos.mr50 {
            if self.pointer > i as usize && self.hist[self.pointer - 1 - i as usize] == pos.hash {
                return true;
            }
        }
        false
    }
}
