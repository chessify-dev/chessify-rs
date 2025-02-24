/// Exhaustive enum of all available piece types.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// The number of different pieces in the game of chess.
pub const NUM_PIECES: usize = 6;
/// An [`array`] containing the pieces with placement corresponding to their respective index.
pub const PIECES: [Piece; NUM_PIECES] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];

impl Piece {
    /// Get the [`usize`] index of the piece.
    ///
    /// This is usually used for efficient table lookups.
    pub fn as_index(&self) -> usize {
        *self as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_index() {
        assert_eq!(0, Piece::Pawn.as_index());
        assert_eq!(1, Piece::Knight.as_index());
        assert_eq!(2, Piece::Bishop.as_index());
        assert_eq!(3, Piece::Rook.as_index());
        assert_eq!(4, Piece::Queen.as_index());
        assert_eq!(5, Piece::King.as_index());
    }

    #[test]
    fn index_lookup() {
        assert_eq!(Piece::Pawn, PIECES[0]);
        assert_eq!(Piece::Rook, PIECES[Piece::Rook.as_index()]);
    }
}
