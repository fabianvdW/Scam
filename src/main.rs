use scam::bitboard::*;
use scam::types::*;
use scam::*;
fn main() {
    let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let pos = position::Position::parse_fen(fen);
    println!(
        "{:b}",
        Magic::bishop_attacks(F1, pos.color_bb[0] | pos.color_bb[1])
    );
    println!("{:b}", Magic::bishop_attacks(F1, BB_ZERO));
    println!(
        "{:b}",
        Magic::rook_attacks(H1, pos.color_bb[0] | pos.color_bb[1])
    );
    println!("{:b}", Magic::rook_attacks(H1, BB_ZERO));
}
