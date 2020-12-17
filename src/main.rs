use scam::position::*;
use scam::r#move::*;
// use scam::*;

fn main() {
    let fen: &str = "r2qk2r/pb2bppp/1p1p4/2p1P2n/5Q2/2N1BP2/PPP1B1PP/R3K2R w Kkq - 0 1";
    let pos = Position::parse_fen(fen);
    let mut mv_list = MoveList::default();
    pos.gen_pseudo_legals(&mut mv_list);
    while let Some((mv, _)) = mv_list.pop() {
        println!("{}", mv);
    }
}
