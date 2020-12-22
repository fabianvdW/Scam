use crate::not;
use crate::types::*;
use std::ops::*;

#[macro_export]
macro_rules! bb {
   ($ ($x: expr), +) => {
        {
            let mut temp = 0;
            $(
                temp |=  1 << $x;
            )*
            BitBoard(temp)
        }
   };
}

pub const BB_ZERO: BitBoard = BitBoard(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn is_empty(self) -> bool {
        self.0 == 0u64
    }

    pub fn not_empty(self) -> bool {
        !self.is_empty()
    }

    pub fn msb(self) -> Square {
        debug_assert!(self.not_empty());
        63 - self.0.leading_zeros()
    }

    pub fn lsb(self) -> Square {
        debug_assert!(self.not_empty());
        self.0.trailing_zeros()
    }

    pub fn pop_lsb(&mut self) -> Square {
        let lsb = self.lsb();
        self.0 &= self.0 - 1u64;
        lsb
    }

    pub const fn popcount(self) -> u32 {
        self.0.count_ones()
    }

    pub const fn shift(self, dir: Direction) -> BitBoard {
        let res = if dir & 7 == 7 {
            self.and(not!(FILE_A_BB))
        } else if dir & 7 == 1 {
            self.and(not!(FILE_H_BB))
        } else {
            self
        };
        if dir > 0 {
            res.shl(dir as u32)
        } else {
            res.shr(-dir as u32)
        }
    }
}

// The code below is only there to use the BitBoard type in const functions,
// as operator overloading not yet allows the trait functions to be const.
// This will (hopefully) be stabilized in future versions of Rust, and
// currently already works on nightly Rust. Once it is stabilized, the impl
// block, the macro and all of its usages will immediately be replaced by the
// overloaded operators. The RFC for this change sits at
// https://github.com/rust-lang/rfcs/pull/2632, and the tracking issue on the
// Rust repository at:  https://github.com/rust-lang/rust/issues/67792
#[macro_export]
macro_rules! not {
    ($x: expr) => {
        BitBoard(!$x.0)
    };
}

impl BitBoard {
    pub const fn shr(self, rhs: u32) -> BitBoard {
        BitBoard(self.0 >> rhs)
    }

    pub const fn shl(self, rhs: u32) -> BitBoard {
        BitBoard(self.0 << rhs)
    }

    pub const fn and(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0 & rhs.0)
    }

    pub const fn or(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0 | rhs.0)
    }

    pub const fn xor(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl Iterator for BitBoard {
    type Item = Square;

    fn next(&mut self) -> Option<Square> {
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

impl Shr<u32> for BitBoard {
    type Output = Self;

    fn shr(self, rhs: u32) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shl<u32> for BitBoard {
    type Output = Self;

    fn shl(self, rhs: u32) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl std::fmt::Binary for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res_str = String::new();
        for &rank in RANK_BB.iter().rev() {
            for &file in FILE_BB.iter() {
                res_str.push_str(&((*self >> (rank & file).lsb()).0 & 1).to_string());
            }
            res_str.push('\n');
        }
        write!(f, "{}", res_str)
    }
}
