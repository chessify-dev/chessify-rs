use crate::bitboard::{Bitboard, EMPTY};
use crate::color::{Color, NUM_COLORS};
use crate::error::{Error, Result};
use crate::piece::NUM_PIECES;
use crate::square::Square;

pub const DEFAULT_BOARD_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Board {
    pieces: [Bitboard; NUM_PIECES * 2],
    side_to_move: Color,
}

impl Board {
    /// Create a new [`Board`] that is completely empty. 
    pub fn empty() -> Self {
        Board {
            pieces: [EMPTY; NUM_PIECES * NUM_COLORS],
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
        BoardBuilder::try_with_fen(fen).unwrap().build()
    }
}

impl Default for Board {
    fn default() -> Self {
        BoardBuilder::try_with_fen(DEFAULT_BOARD_FEN).unwrap().build()
    }
}

pub struct BoardBuilder {
    pieces: Option<[Bitboard; NUM_PIECES * NUM_COLORS]>,
    side_to_move: Option<Color>,
}

impl BoardBuilder {
    ///
    pub fn build(&self) -> Board {
        todo!()
    }

    /// Try to build a board state from a provided FEN string.
    ///
    /// # Errors
    /// If the provided FEN string was invalid.
    ///
    /// # Details
    /// Forsyth–Edwards Notation (FEN) is a standard notation for describing a
    /// particular board position of a chess game. The purpose of FEN is to
    /// provide all the necessary information to restart a game from a particular
    /// position [wikipedia link](https://en.wikipedia.org/wiki/Forsyth–Edwards_Notation). 
    ///
    /// Below you can see the FEN for the starting position:
    /// rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    ///
    /// from which we can see that the FEN contains 6 main fields:
    ///     1. Piece placement information
    ///     2. Active color (player to make a move)
    ///     3. Castling availability
    ///     4. En passant target square
    ///     5. Halfmove clock
    ///     6. Fullmove number
    ///
    pub fn try_with_fen(fen: &str) -> Result<BoardBuilder> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.len() < 4 {
            return Err(Box::new(Error::InvalidFen(fen.to_string())));
        }

        // Initialize board state as empty.
        let mut bitboards: [Bitboard; NUM_PIECES * NUM_COLORS] = [EMPTY; NUM_PIECES * NUM_COLORS];

        // This is the eight rank.
        let mut rank: usize = 7;
        // This is the a file.
        let mut file: usize = 0;

        let piece_placement: &str = parts[0];
        let _active_color: &str = parts[1];
        let _castlng_rights: &str = parts[2];
        let _en_passant_square: &str = parts[3];

        for c in piece_placement.chars() {

            if c == '/' {
                rank -= 1;
                file = 0;
                continue;
            }

            let s: Square = Square::from_idx(rank * 8 + file);
            let bb_idx: usize;

            match c {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    file += c as usize;
                    continue;
                },
                'P' => {
                    bb_idx = 0;
                },
                'N' => {
                    bb_idx = 1;
                },
                'B' => {
                    bb_idx = 2;
                },
                'R' => {
                    bb_idx = 3;
                },
                'Q' => {
                    bb_idx = 4;
                },
                'K' => {
                    bb_idx = 5;
                },
                'p' => {
                    bb_idx = 6;
                },
                'n' => {
                    bb_idx = 7;
                },
                'b' => {
                    bb_idx = 8;
                },
                'r' => {
                    bb_idx = 9;
                },
                'q' => {
                    bb_idx = 10;
                },
                'k' => {
                    bb_idx = 11;
                },
                _ => {
                    return Err(Box::new(Error::InvalidFen(fen.to_string())));
                }
            }

            bitboards[bb_idx] = Bitboard::from_square(s);
        }

        Ok(BoardBuilder{
            pieces: Some(bitboards),
            side_to_move: Some(Color::White),
        })
    }
}

