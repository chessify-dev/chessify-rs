use crate::bitboard::{Bitboard, EMPTY};
use crate::color::{Color, NUM_COLORS};
use crate::error::{Error, Result};
use crate::piece::{NUM_PIECES, Piece};
use crate::square::Square;

use std::collections::HashMap;
use std::fmt;

pub const DEFAULT_BOARD_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Board {
    bitboards: [Bitboard; NUM_PIECES * 2],
    pieces: HashMap<usize, (Piece, Color)>,
    side_to_move: Color,
}

impl Board {
    /// Create a new [`Board`] that is completely empty. 
    pub fn empty() -> Self {
        Board {
            bitboards: [EMPTY; NUM_PIECES * NUM_COLORS],
            pieces: HashMap::new(),
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
        BoardBuilder::try_with_fen(fen).unwrap().try_build().unwrap()
    }

    pub fn print_bitboards(&self) {
        for piece in 0..12 {
            println!("{}", self.bitboards[piece]);
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        BoardBuilder::try_with_fen(DEFAULT_BOARD_FEN).unwrap().try_build().unwrap()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for rank in 0..8 {
            for field in 0..8 {
                if field == 0 {
                    write!(f, " {} ", 8 - rank)?;
                }

                if let Some((piece, color)) = self.pieces.get(&(rank * 8 + field)) {
                    write!(f, " {} ", piece.to_string(*color))?;
                } else {
                    write!(f, " . ")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "    a b c d e f g h")
    }
}

pub struct BoardBuilder {
    bitboards: Option<[Bitboard; NUM_PIECES * NUM_COLORS]>,
    pieces: Option<HashMap<usize, (Piece, Color)>>,
    side_to_move: Option<Color>,
}

impl BoardBuilder {
    ///
    pub fn try_build(self) -> Result<Board> {
        let bitboards: [Bitboard; NUM_PIECES * NUM_COLORS] = self.bitboards.ok_or_else(|| {
            Box::new(Error::BoardSetup("Bitboards not initialized".to_string()))
        })?;

        let pieces: HashMap<usize, (Piece, Color)> = self.pieces.ok_or_else(|| {
            Box::new(Error::BoardSetup("Pieces HashMap not initialized".to_string()))
        })?;

        let side_to_move: Color = self.side_to_move.ok_or_else(|| {
            Box::new(Error::BoardSetup("Side to move not initialized".to_string()))
        })?;

        Ok(Board {
            bitboards,
            pieces,
            side_to_move,
        })
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
        let mut pieces: HashMap<usize, (Piece, Color)> = HashMap::new();

        // In reality we are actually going from the 8th rank to the 1st rank,
        // but we calculate this counter backwards because its easier with indices...
        let mut rank: usize = 0;
        let mut file: usize = 0;

        let piece_placement: &str = parts[0];
        let _active_color: &str = parts[1];
        let _castlng_rights: &str = parts[2];
        let _en_passant_square: &str = parts[3];

        for c in piece_placement.chars() {

            if c == '/' {
                rank += 1;
                file = 0;
                continue;
            }

            let s: Square = Square::from_idx(rank * 8 + file);
            let bb_idx: usize;

            let piece: Piece;
            let color: Color;

            match c {
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                    file += c as usize;
                    continue;
                },
                'P' => {
                    bb_idx = 0;
                    piece = Piece::Pawn;
                    color = Color::White;
                },
                'N' => {
                    bb_idx = 1;
                    piece = Piece::Knight;
                    color = Color::White;
                },
                'B' => {
                    bb_idx = 2;
                    piece = Piece::Bishop;
                    color = Color::White;
                },
                'R' => {
                    bb_idx = 3;
                    piece = Piece::Rook;
                    color = Color::White;
                },
                'Q' => {
                    bb_idx = 4;
                    piece = Piece::Queen;
                    color = Color::White;
                },
                'K' => {
                    bb_idx = 5;
                    piece = Piece::King;
                    color = Color::White;
                },
                'p' => {
                    bb_idx = 6;
                    piece = Piece::Pawn;
                    color = Color::Black;
                },
                'n' => {
                    bb_idx = 7;
                    piece = Piece::Knight;
                    color = Color::Black;
                },
                'b' => {
                    bb_idx = 8;
                    piece = Piece::Bishop;
                    color = Color::Black;
                },
                'r' => {
                    bb_idx = 9;
                    piece = Piece::Rook;
                    color = Color::Black;
                },
                'q' => {
                    bb_idx = 10;
                    piece = Piece::Queen;
                    color = Color::Black;
                },
                'k' => {
                    bb_idx = 11;
                    piece = Piece::King;
                    color = Color::Black;
                },
                _ => {
                    return Err(Box::new(Error::InvalidFen(fen.to_string())));
                }
            }

            bitboards[bb_idx] |= Bitboard::from_square(s);
            pieces.insert(rank * 8 + file, (piece, color));
            file += 1;
        }

        Ok(BoardBuilder{
            bitboards: Some(bitboards),
            pieces: Some(pieces),
            side_to_move: Some(Color::White),
        })
    }
}

