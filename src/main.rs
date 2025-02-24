use chessify::{Bitboard, Board};

fn main() {
    let b: Board = Board::default();
    println!("{}", b);
    b.print_bitboards();
}
