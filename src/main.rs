use scam::attacks::*;
use scam::bitboard::*;
use scam::types::*;
use scam::*;

fn main() {
    let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let pos = position::Position::parse_fen(fen);

    for pt in [KNIGHT, BISHOP, ROOK, QUEEN, KING].iter() {
        println!("{:b}", attack_bb(*pt, H3, BitBoard(0)));
    }

    println!("{:b}", pawn_attack_bb(WHITE, H3));

    let func = pawn_bb_attack_bb; // cool
    println!("{:b}", func(WHITE, pos.piece_bb[PAWN as usize]));
}
