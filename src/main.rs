use scam::*;

fn main() {
    let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    position::Position::parse_fen(fen);

}
