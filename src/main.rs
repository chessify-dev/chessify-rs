use chessify::Board;

fn main() {
    let b = Board::try_from_fen("a/b/c/d/b/c a b d e");
    println!("{:?}", b);
}
