use scam::position::*;
use scam::types::*;
// use scam::*;

fn main() {
    let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let pos = Position::parse_fen(fen);
    println!("{}", pos.in_check(WHITE));

    let fen = "rnbqkbnr/ppp1pppp/3p4/8/8/7K/PPPPPPPP/RNBQ1BNR w kq - 0 1";
    let pos = Position::parse_fen(fen);
    println!("{}", pos.in_check(WHITE));
}
