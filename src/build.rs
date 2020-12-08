#[macro_use]
pub mod bitboard;
pub mod types;
use bitboard::*;
use types::*;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let has_bmi2 = env::var("CARGO_CFG_TARGET_FEATURE").map_or(false, |x| x.contains("bmi2"));
    let magic_path = Path::new(&out_dir).join("magic_attacks.rs");
    let mut file = File::create(magic_path).unwrap();
    if has_bmi2 {
        writeln!(file, "//Tables for BMI2 has_bmi2").unwrap();
    } else {
        writeln!(file, "//Tables for Magics has_bmi2").unwrap();
    }
    let attacks = initialize_attacks(has_bmi2);
    write!(file, "{}", print_attacks(attacks)).unwrap();
}

pub fn print_attacks(attacks: Vec<BitBoard>) -> String {
    let mut res_str = String::new();
    res_str.push_str("#[rustfmt::skip]\npub static ATTACKS : [u64; 107648] = [");
    for &attack in attacks.iter() {
        res_str.push_str(&format!("{},", attack.0));
    }
    res_str.push_str("];");
    res_str
}

pub fn slider_attacks(sq: Square, attack_dirs: &[Direction; 4], occ: BitBoard) -> BitBoard {
    let mut res = BB_ZERO;
    for &dir in attack_dirs.iter() {
        let mut temp = bb!(sq);
        for _ in 0..6 {
            temp |= temp.shift(dir) & !occ;
        }
        res |= temp.shift(dir);
    }
    res
}

impl Magic {
    pub fn build_index(self, occ: BitBoard, has_bmi2: bool) -> usize {
        if has_bmi2 {
            use std::arch::x86_64::_pext_u64;
            self.offset + unsafe { _pext_u64(occ.0, self.mask.0) } as usize
        } else {
            self.offset + (((occ & self.mask).0).wrapping_mul(self.magic) >> self.shift) as usize
        }
    }
}

pub fn initialize_attacks(has_bmi2: bool) -> Vec<BitBoard> {
    let mut res = vec![BitBoard(0); 107648];
    for (magics, dirs) in [(BISHOP_MAGICS, BISHOP_DIRS), (ROOK_MAGICS, ROOK_DIRS)].iter() {
        for (sq, magic) in magics.iter().enumerate() {
            let mut occ = BB_ZERO;
            loop {
                let attacks = slider_attacks(sq as Square, dirs, occ);
                res[magic.build_index(occ, has_bmi2)] = attacks;
                occ = BitBoard((occ.0.wrapping_sub(magic.mask.0)) & magic.mask.0);
                if occ.is_empty() {
                    break;
                }
            }
        }
    }
    res
}
