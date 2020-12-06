use scam::*;

fn main() {
    let fen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    let pos = position::Position::parse_fen(fen);

    println!("{:b}", pos.color_bb[0].shift(types::NORTH));
    println!("{:b}", pos.color_bb[0].shift(types::NORTH_EAST));
    println!("{:b}", pos.color_bb[1].shift(types::SOUTH_EAST));
    print!("{:b}", pos.color_bb[1].shift(types::SOUTH));
}
