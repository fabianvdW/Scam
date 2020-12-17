use scam::position::*;
use scam::r#move::*;
// use scam::*;

fn main() {
    let fen: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    let pos = Position::parse_fen(fen);
    let mut mv_list = MoveList::default();
    pos.gen_pseudo_legals(&mut mv_list);
    let mut i = 1;
    while let Some((mv, _)) = mv_list.pop() {
        println!("{}: {}", i, mv);
        i += 1;
    }
}
