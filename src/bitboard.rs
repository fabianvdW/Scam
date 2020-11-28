use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn is_empty(self) -> bool {
        self.0 == 0u64
    }

    pub fn not_empty(self) -> bool {
        !self.is_empty()
    }

    pub fn lsb(self) -> u32 {
        debug_assert!(self.not_empty());
        self.0.trailing_zeros()
    }

    pub fn pop_lsb(&mut self) -> u32 {
        let lsb = self.lsb();
        self.0 &= self.0 - 1u64;
        lsb
    }

    pub fn popcount(self) -> u32 {
        self.0.count_ones()
    }
}

impl Iterator for BitBoard {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.is_empty() {
            return None;
        }
        Some(self.pop_lsb())
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

impl Shr<i32> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: i32) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs >> rhs)
    }
}

impl Shl<i32> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: i32) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs << rhs)
    }
}
