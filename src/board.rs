use crate::bitboard::{Bitboard, EMPTY};
use crate::color::Color;
use crate::error::Result;
use crate::piece::NUM_PIECES;

pub const DEFAULT_BOARD_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Board {
    pieces: [Bitboard; NUM_PIECES * 2],
    side_to_move: Color,
}

impl Board {
    /// Create a new [`Board`] that is completely empty. 
    pub fn empty() -> Self {
        Board {
            pieces: [EMPTY; NUM_PIECES * 2],
            side_to_move: Color::White,
        }
    }

    /// Alias for [`Board::empty()`] which creates a new empty board.
    ///
    /// If you want to create a board with the standard starting position,
    /// then use either of [`Board::default()`] or [`Board::standard()`].
    pub fn new() -> Self {
        Board::empty()
    }

    /// Create a new [`Board`] with the default starting position.
    pub fn standard() -> Self {
        Board::default()
    }

    /// Create a new [`Board`] from a user specified Forsyth-Edwards-Notation (FEN)
    /// string. 
    ///
    /// # Panics
    /// Iff the user provided an invalid FEN string.
    pub fn from_fen(fen: &str) -> Self {
        BoardBuilder::with_fen(fen).unwrap().build()
    }
}

impl Default for Board {
    fn default() -> Self {
        BoardBuilder::with_fen(DEFAULT_BOARD_FEN).unwrap().build()
    }
}

pub struct BoardBuilder {
    pieces: [Bitboard; NUM_PIECES * 2],
}

impl BoardBuilder {
    ///
    pub fn build(&self) -> Board {
        todo!()
    }

    ///
    pub fn with_fen(fen: &str) -> Result<BoardBuilder> {
        let parts: Vec<&str> = fen.split_whitespace().collect();

        todo!()
    }
}
