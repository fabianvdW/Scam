use crate::bitboard::*;
use crate::types::*;

#[derive(Clone, Copy)]
pub struct Magic {
    pub occ_mask: BitBoard,
    pub magic_number: u64,
    pub offset: usize,
    pub shift: u32,
}

impl Magic {
    pub const fn default() -> Magic {
        Magic {
            occ_mask: BB_ZERO,
            magic_number: 0,
            offset: 0,
            shift: 0,
        }
    }
    #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
    #[inline(always)]
    pub fn apply(&self, occ: BitBoard) -> BitBoard {
        use std::arch::x86_64::_pext_u64;
        ATTACKS[self.offset + unsafe { _pext_u64(occ.0, self.occ_mask.0) } as usize]
    }

    #[cfg(not(all(target_arch = "x86_64", target_feature = "bmi2")))]
    #[inline(always)]
    pub const fn apply(&self, occ: BitBoard) -> BitBoard {
        ATTACKS[(self.offset
            + (((occ & self.occ_mask).0).wrapping_mul(self.magic_number) >> (64 - self.shift))
                as usize)]
    }
}

pub const fn occupancy_mask(sq: Square, attack_dirs: [Direction; 4]) -> BitBoard {
    let mut res = BB_ZERO;
    let mut i = 0;
    while i < 4 {
        let mut j = 0;
        let dir = attack_dirs[i];
        let mut temp = bb!(sq).shift(dir);
        while j < 5 {
            let mut next_attack = temp.or(temp.shift(dir));
            //Need to exclude certain last squares from occupancy masks.
            if dir >= 7 {
                next_attack = next_attack.and(not!(RANK_8_BB))
            }
            if dir <= -7 {
                next_attack = next_attack.and(not!(RANK_1_BB))
            }
            if dir & 7 == 1 {
                next_attack = next_attack.and(not!(FILE_H_BB))
            }
            if dir & 7 == 7 {
                next_attack = next_attack.and(not!(FILE_A_BB))
            }
            temp = next_attack;
            j += 1;
        }
        res = res.or(temp);
        i += 1;
    }
    res
}

pub const fn initialize_magics(
    magic_nums: [u64; 64],
    attack_dirs: [Direction; 4],
    offset: usize,
) -> [Magic; 64] {
    let mut magics = [Magic::default(); 64];
    let mut sq = 0;
    while sq < SQUARE_NB {
        magics[sq].magic_number = magic_nums[sq];
        magics[sq].occ_mask = occupancy_mask(sq as Square, attack_dirs);
        magics[sq].shift = magics[sq].occ_mask.popcount();
        magics[sq].offset = if sq == 0 {
            offset
        } else {
            magics[sq - 1].offset + (1 << magics[sq - 1].shift)
        };
        sq += 1;
    }
    magics
}
//Setting up Bishop magics
#[rustfmt::skip]
pub const BISHOP_MAGIC_NUMBERS : [u64;64] = [9052302183530624u64, 3493106745918750722u64, 10378547575765598208u64, 1737267960881348624u64, 10173093901303832u64, 4666011823880819232u64, 595602155869570050u64, 4611897984056627264u64, 36249008850862404u64, 2216337449216u64, 2305851882628841472u64, 184651999957483520u64, 7494011856613818624u64, 1197984168606171392u64, 2256765064877074u64, 147774504575173632u64, 9232379519711904000u64, 1589780154182344962u64, 5843420671266299912u64, 2306970043015012930u64, 291610284032786432u64, 1412881035952660u64, 18577349571281920u64, 288265571328395280u64, 20398418977359873u64, 4616194017980600448u64, 2308105804345245712u64, 4611826893489045536u64, 9009398294841476u64, 2634606881531924610u64, 283674285703424u64, 1261300437177876736u64, 19333830213640194u64, 9225705209122014272u64, 36314674337548288u64, 5188148971919900801u64, 16289522094180425736u64, 81082939529527360u64, 5198622926656012808u64, 9656916352225543296u64, 2261180160746545u64, 40818338457190912u64, 1152932510729241088u64, 148919646538486784u64, 10134203572167168u64, 1135797138883072u64, 164383759939144704u64, 9233225930963681536u64, 100207325126067208u64, 1153207386539033088u64, 4611361466745472u64, 57139560060289058u64, 288248037091186432u64, 1301584865623408704u64, 75611525158570019u64, 146384586526490896u64, 1164255287713071617u64, 288338171259344900u64, 5764607534879117377u64, 1157495747864957184u64, 3222077704u64, 4616752605052544032u64, 2343072610411356416u64, 73218686973968530u64, ];
pub const BISHOP_MAGICS: [Magic; 64] = initialize_bishop_magics();
pub const fn initialize_bishop_magics() -> [Magic; 64] {
    initialize_magics(BISHOP_MAGIC_NUMBERS, BISHOP_DIRS, 0)
}

//Setting up Rook magics
#[rustfmt::skip]
pub const ROOK_MAGIC_NUMBERS : [u64;64] = [2630106718943609138u64, 18032010559799296u64, 180161586023891074u64, 2449967268337156128u64, 36037593179127810u64, 1297037861652529664u64, 216173881668150784u64, 144115755014179329u64, 9516246750663278729u64, 2392674749399056u64, 14777779876790404u64, 578853461412548608u64, 36169551687712896u64, 4925820690762752u64, 422225358362112u64, 10387834016590004226u64, 468374636126535876u64, 2305918051150733312u64, 1153062792119508996u64, 40532946536465424u64, 5770519597325746180u64, 9223662312756613184u64, 36103566096597521u64, 9228176902740050052u64, 1242995973202911360u64, 301811597467189376u64, 3103015342663795328u64, 5944769102463107204u64, 5764629515414798465u64, 3458766714999669760u64, 288232592363292688u64, 290483284066992324u64, 351855003566724u64, 1371381339630076098u64, 2307021687834566656u64, 576496040862028288u64, 2955521640369152u64, 24910690066104832u64, 149602367980033u64, 140738620818688u64, 140738562129952u64, 4620836158493032480u64, 1157636347922546704u64, 4802950260195336u64, 8800388317200u64, 297959129979814176u64, 9017713502715912u64, 360429292935315457u64, 2306267730627658240u64, 666534181534443776u64, 360596933493932288u64, 288250168435319296u64, 7036908795364608u64, 2307531895849878016u64, 864708755556017152u64, 11608168789920731776u64, 144255964230459459u64, 4719808153548554754u64, 36117037123772417u64, 4756118072021484801u64, 581245895669196801u64, 563037070164226u64, 4684025104663969825u64, 2256199512819778u64, ];
pub const ROOK_MAGICS: [Magic; 64] = initialize_rook_magics();
pub const fn initialize_rook_magics() -> [Magic; 64] {
    initialize_magics(
        ROOK_MAGIC_NUMBERS,
        ROOK_DIRS,
        BISHOP_MAGICS[SQUARE_NB - 1].offset + (1 << BISHOP_MAGICS[SQUARE_NB - 1].shift),
    )
}

//Setting up attack tables
pub const ATTACKS: [BitBoard; 107648] = initialize_attacks();

pub const fn slider_attacks(sq: Square, attack_dirs: [Direction; 4], occ: BitBoard) -> BitBoard {
    let mut res = BB_ZERO;
    let mut i = 0;
    while i < 4 {
        let mut temp = bb!(sq);
        let mut j = 0;
        while j < 7 {
            temp = temp.or(temp.shift(attack_dirs[i]));
            if j < 6 {
                temp = temp.and(not!(occ));
            }
            j += 1;
        }
        res = res.or(temp);
        i += 1;
    }
    res.and(not!(bb!(sq)))
}
pub const fn initialize_attacks() -> [BitBoard; 107648] {
    let mut res = [BitBoard(0); 107648];
    let sliders = [(BISHOP_MAGICS, BISHOP_DIRS), (ROOK_MAGICS, ROOK_DIRS)];
    let mut slider = 0;
    while slider < 2 {
        let mut sq = 0;
        while sq < SQUARE_NB {
            let magic = sliders[slider].0[sq];
            let mut pattern = 0;
            while pattern < (1 << magic.shift) {
                let occ_pattern = BitBoard(pdep(magic.occ_mask.0, pattern));
                let attacks = slider_attacks(sq as Square, sliders[slider].1, occ_pattern);
                #[cfg(not(all(target_arch = "x86_64", target_feature = "bmi2")))]
                {
                    res[magic.offset + magic.apply(occ_pattern)] = attacks;
                }
                #[cfg(all(target_arch = "x86_64", target_feature = "bmi2"))]
                {
                    res[magic.offset + pattern as usize] = attacks;
                }
                pattern += 1;
            }
            sq += 1;
        }
        slider += 1;
    }
    res
}

pub const fn pdep(mut mask: u64, temp: u64) -> u64 {
    let mut res = 0u64;
    let mut temp_index = 0;
    while mask > 0u64 {
        let idx = mask.trailing_zeros();
        if (temp & (1 << temp_index)) > 0 {
            res |= 1 << idx;
        }
        temp_index += 1;
        mask ^= 1 << idx;
    }
    res
}
