use crate::position::Position;

#[derive(Default, Clone)]
pub struct HashHist {
    hist: Vec<u64>,
}
impl HashHist {
    pub fn clear(&mut self) {
        self.hist.clear();
    }

    pub fn push(&mut self, pos: &Position) {
        self.hist.push(pos.hash);
    }

    pub fn pop(&mut self) {
        self.hist.pop();
    }

    pub fn is_2fold(&self, pos: &Position) -> bool {
        for i in 1..=pos.mr50 {
            if self.hist.len() > i as usize
                && self.hist[self.hist.len() - 1 - i as usize] == pos.hash
            {
                return true;
            }
        }
        false
    }
}
