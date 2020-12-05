use crate::constants::*;
use crate::position::{Color, WHITE};
use std::ops::*;

#[macro_export]
macro_rules! bb {
   ($ ($x: expr), *) => {
        {
            let mut temp = 0;
            $(
                temp |=  1 << $x;
            )*
            BitBoard(temp)
        }
   };
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BitBoard(pub u64);

pub type Direction = i8;
pub const NORTH: Direction = 8;
pub const SOUTH: Direction = -8;
pub const EAST: Direction = 1;
pub const WEST: Direction = -1;
pub const NORTH_EAST: Direction = 9;
pub const NORTH_WEST: Direction = 7;
pub const SOUTH_EAST: Direction = -7;
pub const SOUTH_WEST: Direction = -9;
pub const fn relative_dir(dir: Direction, color: Color) {
    if color == WHITE {
        dir
    } else {
        -dir
    };
}
impl BitBoard {
    pub const fn shift(self, dir: Direction) -> BitBoard {
        let res = if dir & 7 == 7 {
            self & !FILE_A
        } else if dir & 7 == 1 {
            self & !FILE_H
        } else {
            self
        };
        if dir > 0 {
            res << (dir as u32)
        } else {
            res << (-dir as u32)
        }
    }
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

impl const Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl const BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl const BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl const BitXor for BitBoard {
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

impl const Shr<u32> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs >> rhs)
    }
}

impl const Shl<u32> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        let Self(lhs) = self;
        Self(lhs << rhs)
    }
}

impl std::fmt::Binary for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res_str = String::new();
        for &rank in RANKS.iter().rev() {
            for &file in FILES.iter() {
                res_str.push_str(&((*self >> (rank & file).lsb()).0 & 1).to_string());
            }
            res_str.push('\n');
        }
        write!(f, "{}", res_str)
    }
}
