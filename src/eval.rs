use crate::position::Position;
use crate::types::*;
use std::convert::TryFrom;

#[rustfmt::skip]
pub const PSQT: [[i32; 64]; 15] = [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ], [23938, 21708, 25602, 28660, 25511, 27675, 25302, 25657, 62034, 63717, 47340, 48230, 46840, 72538, 76646, 55768, 64452, 69096, 57425, 61235, 58369, 60959, 73962, 57970, 64850, 72285, 65906, 63500, 62485, 57962, 70884, 60961, 75762, 82160, 76276, 66433, 79297, 74807, 74948, 59151, 113000, 97831, 117964, 108656, 114268, 106816, 106522, 83569, 161572, 167501, 148400, 137111, 133244, 125323, 110442, 126491, 24338, 27011, 27259, 28703, 22727, 24638, 24484, 25772, ], [68557, 64488, 51255, 51379, 60666, 51889, 67724, 14901, 57725, 66780, 70292, 85892, 96630, 83350, 67670, 63227, 71673, 87425, 96561, 106487, 105446, 97706, 106368, 72554, 85138, 86372, 114049, 113061, 114687, 115311, 118256, 90866, 95053, 111699, 130174, 129795, 125735, 147683, 115801, 127957, 106525, 112739, 131670, 135391, 140621, 140542, 124789, 101861, 91126, 113634, 122886, 125384, 133242, 117083, 97696, 75458, 74469, 104534, 105932, 103020, 104433, 104328, 95354, 36231, ], [96078, 111873, 94330, 94123, 85368, 98512, 101166, 92468, 122319, 119208, 126327, 107213, 111331, 104908, 124393, 114349, 117768, 132527, 129535, 123427, 128783, 127284, 126854, 117100, 126438, 120963, 136590, 137203, 133326, 128908, 122390, 120206, 111270, 133141, 131231, 148742, 147550, 144326, 126446, 124217, 110762, 140764, 129081, 149848, 150924, 142381, 156216, 137046, 113065, 130470, 144423, 143444, 129185, 148432, 112385, 115608, 107320, 134494, 118864, 121424, 128904, 104281, 133388, 134759, ], [192561, 197052, 202017, 207811, 205024, 200055, 195878, 184599, 176166, 181307, 186756, 190229, 194227, 188716, 201075, 189398, 181151, 190210, 192234, 194349, 197890, 193459, 204558, 192870, 206507, 211630, 207720, 214846, 213879, 206386, 212840, 188776, 220748, 221480, 235653, 236474, 232742, 234747, 223503, 221917, 233010, 239816, 243572, 247959, 243707, 250955, 234397, 225052, 240399, 237122, 242026, 240777, 245952, 231967, 233316, 232372, 244614, 234629, 231157, 244034, 239805, 234217, 240511, 234357, ], [368376, 366117, 367382, 368055, 357885, 333163, 365084, 347368, 358210, 369169, 375770, 367860, 367455, 365855, 366199, 365669, 359545, 370781, 376296, 369566, 367636, 373420, 382265, 354527, 358678, 372913, 376329, 388303, 384100, 391284, 377105, 379271, 371014, 368673, 378840, 399506, 397168, 418374, 403202, 394793, 353620, 365664, 386664, 394857, 411230, 433690, 427196, 422281, 361360, 371385, 391019, 411780, 403554, 410646, 417879, 426636, 384466, 388554, 389066, 402057, 412987, 420606, 424395, 409045, ], [-2700, 7525, 935, -33703, -36870, -26704, -6476, -4371, 7334, -1428, -11960, -4890, -4575, -6791, -1951, -3679, -6069, 2538, -447, 7864, 6461, 3624, 6494, -5690, -10479, 10957, 8396, 19566, 20035, 5658, 14283, -7853, 6696, 21085, 27955, 13396, 22818, 20253, 23428, 2573, 28481, 24873, 21775, 2471, 14471, 31943, 26017, 10673, 125, 29619, 28458, 46923, 21115, 32822, 28885, 18061, -21132, 14816, 11776, 27909, 18616, -959, 21188, 16853, ], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, ], [-27050, -24919, -30539, -31509, -27682, -26142, -26668, -31486, -168248, -158153, -152815, -132928, -130507, -121139, -125497, -129217, -102272, -101642, -109389, -113809, -111945, -107198, -98748, -91369, -77892, -83012, -78636, -72735, -76467, -74395, -75390, -69065, -68175, -71884, -70756, -63993, -67565, -59913, -73992, -61172, -69358, -64554, -58715, -54322, -56141, -57782, -73372, -62272, -57993, -66431, -55227, -50038, -49911, -73536, -76898, -59387, -28183, -22489, -22523, -30847, -23831, -26040, -30355, -28875, ], [-63158, -68651, -105301, -121250, -119232, -91480, -96917, -40840, -80607, -110788, -112309, -125051, -127037, -128793, -111893, -96845, -96254, -106194, -133495, -133945, -147139, -145354, -128607, -100316, -101189, -110811, -132910, -128758, -129440, -147009, -114691, -111276, -85221, -85526, -117073, -107494, -112330, -114456, -110361, -87134, -71086, -87352, -94397, -99186, -111303, -97465, -86555, -63472, -53774, -50223, -78717, -86142, -87590, -79401, -75398, -64466, -38082, -65636, -33411, -59177, -75123, -43834, -64615, -22487, ], [-102270, -133898, -137342, -127241, -133349, -92814, -137676, -120040, -108560, -135231, -135053, -135200, -135225, -135204, -116910, -123873, -124258, -140764, -140288, -136979, -143245, -143121, -153473, -140303, -118538, -132170, -135950, -147975, -146799, -142803, -123379, -131004, -110059, -129288, -127149, -139631, -141255, -134028, -118449, -112132, -118488, -120117, -126291, -124542, -128992, -131152, -127434, -111769, -114340, -118701, -118899, -110386, -113750, -121121, -126679, -100578, -100606, -115839, -95748, -100536, -84255, -98673, -86390, -90574, ], [-238323, -235192, -235858, -237536, -240434, -231354, -235148, -239610, -236222, -233587, -244472, -247704, -247130, -231126, -235595, -235378, -235624, -230022, -238890, -239114, -250718, -245398, -249227, -229846, -220233, -228151, -233585, -245613, -233587, -237253, -228246, -215840, -194516, -210290, -214947, -206546, -196169, -203676, -218998, -198547, -172807, -189194, -196212, -197629, -195670, -198891, -209092, -179507, -169690, -180648, -187741, -189503, -193768, -197657, -184362, -176710, -190531, -196055, -197325, -202059, -205698, -198971, -194707, -184067, ], [-397466, -396211, -412332, -422436, -413195, -419871, -421162, -412276, -358051, -361848, -384790, -405830, -401888, -406265, -401693, -427579, -368320, -379361, -372761, -395558, -422290, -430280, -419887, -406882, -362325, -371197, -390919, -389656, -403126, -411373, -388581, -387372, -370112, -379801, -375738, -379479, -380630, -374422, -381670, -383757, -363100, -370022, -371528, -367101, -366987, -369245, -379906, -350814, -352692, -361149, -363540, -361256, -359069, -364508, -364150, -332190, -372985, -348491, -354453, -361672, -344156, -311936, -333560, -332830, ], [10805, 6428, -29015, -32751, -19802, -44687, -15884, -12257, -14177, -37022, -28287, -31218, -26411, -31098, -44018, -31538, -19463, -28003, -18862, -10668, -9928, -21038, -30389, -14217, -11208, -25894, -24529, -21786, -23163, -25268, -30526, -5831, 14655, -15989, -14869, -18152, -16369, -15638, -4029, 15333, 31671, 9731, -11455, -8884, -4777, -2216, -2460, 12466, -1036, 6023, 16064, 7755, 5529, 10308, 7464, 6257, 15107, -1811, 1796, 41766, 30423, 29048, 8410, 9300, ], ];
pub const TEMPO_BONUS: i32 = 8667;
pub const BIAS: i32 = 9552;
pub const DIV: i32 = 512;
//Params scaled by 2**17 => Evaluation scaled by 2**17/2**9 = 2**8 = 256

pub fn eval(pos: &Position) -> Score {
    let mut eval = pos.piece_eval + BIAS;
    eval = if pos.ctm == WHITE { eval } else { -eval } + TEMPO_BONUS;
    let res = eval / DIV;
    debug_assert!(i16::try_from(res).is_ok()); //Checks that the eval actually within an i16
    res as Score
}
