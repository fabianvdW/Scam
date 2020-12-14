use scam::attacks::*;
use scam::bitboard::*;
use scam::r#move::*;
use scam::types::*;
use scam::*;

fn main() {
    //let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    //let pos = position::Position::parse_fen(fen);

    println!("{}", Move::new(A2, A4, NORMAL, None));
    println!("{}", Move::new(A7, A8, PROMOTION, Some(QUEEN)));
    println!("{}", Move::new(A7, A8, PROMOTION, Some(KNIGHT)));
    println!("{}", Move::new(A7, A8, PROMOTION, Some(BISHOP)));
    println!("{}", Move::new(A7, A8, PROMOTION, Some(ROOK)));
}
