use crate::position::Position;
use crate::r#move::*;
use crate::transposition::hash::{up_hash, BITS_USIZE};
use crate::types::{score_to_tt, Score};

pub mod hash {
    pub const BITS_USIZE: u32 = 8 * std::mem::size_of::<usize>() as u32;
    pub const UP_HASH: u64 = 0xFFFFFFFF00000000;
    pub const fn up_hash(hash: u64) -> u32 {
        ((hash & UP_HASH) >> 32) as u32
    }

    pub static PIECES: [[u64; 64]; 15] = {
        let mut res = [[0; 64]; 15];
        let mut seed = 1070372u64;
        let mut row = 0;
        while row < 15 {
            let mut col = 0;
            while col < 64 {
                // http://vigna.di.unimi.it/ftp/papers/xorshift.pdf
                seed ^= seed >> 12;
                seed ^= seed << 25;
                seed ^= seed >> 27;
                res[row][col] = seed.wrapping_mul(2685821657736338717u64);
                col += 1;
            }
            row += 1;
        }
        res
    };
    pub const CTM: u64 = 13442441245975073873;
    #[rustfmt::skip]
    pub const CASTLE_RIGHTS: [u64; 16] = [0, 2813347996350729101, 4323917982890274150, 17639880714067218301, 6192459193442042619, 10431259969583463413, 6842244024206355455, 16007120865564466847, 1121671930296507050, 15415905829452006450, 9786278101810930012, 4806855920115420048, 11019280650726627950, 17847013704274833886, 5689581433056223789, 18324940204714547484];
    #[rustfmt::skip]
    pub const EP: [u64; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        337906396070687061, 15132773131322557940, 5615115819428020346, 3978109363184239885, 16147680783334850494, 8090450156041063312, 16217433211119228729, 11271410421935427322,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        4981899350120205529, 16853551438605610893, 7075712930220690310, 13820395515365100069, 6357982281089732695, 4489700579363491440, 4808990915787598934, 7860507183168892681,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
}

pub const DEFAULT_TT_SIZE: usize = 8; //in mb

pub const FLAG_EXACT: u8 = 0x1;
pub const FLAG_UPPER: u8 = 0x2;
pub const FLAG_LOWER: u8 = 0x3;

#[rustfmt::skip]
#[derive(Clone, Default)]
#[repr(C, align(16))]
pub struct TTEntry {
    pub up_hash: u32, //4 byte
    pub mv: Move,     //2 byte
    pub score: Score, //2 byte
    pub depth: u8,    //1 byte
    pub flag: u8,    //1 byte
                // Sum: 10 byte
           //Allocated: 16 byte LUL
           //-> Relying on the fact that writes are atomic
           // such that we can assume the mv: Move corresponds
           // to a legal move atleast in some position
           // This excludes moves such as Qa1b3
}
impl TTEntry {
    pub fn is_some(&self) -> bool {
        self.mv != NO_MOVE
    }

    pub fn is_hit(&self, pos: &Position) -> bool {
        self.up_hash == up_hash(pos.hash)
    }

    pub fn is_lower(&self) -> bool {
        self.flag == FLAG_LOWER
    }

    pub fn is_exact(&self) -> bool {
        self.flag == FLAG_EXACT
    }

    pub fn is_upper(&self) -> bool {
        self.flag == FLAG_UPPER
    }
}

#[derive(Default)]
pub struct TT {
    entries: Vec<TTEntry>,
    index_mask: usize,
}

impl TT {
    pub fn allocate(&mut self, size_in_mb: usize) {
        let mut entries: usize = size_in_mb * 1024 * 1024 / 16;
        assert_ne!(entries, 0);
        entries = 1 << (BITS_USIZE - 1 - entries.leading_zeros()); //Round down to nearest integer of power 2
        self.entries = vec![TTEntry::default(); entries as usize];
        self.index_mask = entries - 1;
    }

    pub fn read(&self, pos: &Position) -> &TTEntry {
        &self.entries[pos.hash as usize & self.index_mask]
    }

    pub fn insert(
        &mut self,
        pos: &Position,
        score: Score,
        height: u8,
        mv: Move,
        depth: u8,
        flag: u8,
    ) {
        let entry = &mut self.entries[pos.hash as usize & self.index_mask];
        entry.up_hash = up_hash(pos.hash);
        entry.mv = mv;
        entry.score = score_to_tt(score, height);
        entry.depth = depth;
        entry.flag = flag;
    }
}
