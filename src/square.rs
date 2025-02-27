use crate::error::{ChessifyError, Result};

use std::fmt;

/// Chess square implementation using an unsigned char ([`u8`]). 
///
/// You can either create a [`Square`] by providing its representative board index
/// (0-63) or by supplying a string which follows the standard chess notation.
///
/// The squares on the chess board are organized accordingly:
///
///  a8 b8 c8 d8 e8 f8 g8 h8
///  a7 b7 c7 d7 e7 f7 g7 h7
///  a6 b6 c6 d6 e6 f6 g6 h6
///  a5 b5 c5 d5 e5 f5 g5 h5
///  a4 b4 c4 d4 e4 f4 g4 h4
///  a3 b3 c3 d3 e4 f4 g4 h4
///  a2 b2 c2 d2 e2 f2 g2 h2
///  a1 b1 c1 d1 r1 f1 g1 h1
///
///  where a8 has index 0 and h1 has index 63, i.e., indices go from top to bottom + left to right.
///
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Square(pub u8);

/// Implementation of a file on the chess board (vertically from 0 to 7).
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd)]
pub struct File(pub u8);

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &str = match self.0 {
            0 => "a",
            1 => "b",
            2 => "c",
            3 => "d",
            4 => "e",
            5 => "f",
            6 => "g",
            7 => "h",
            _ => { return Err(fmt::Error); },
        };
        write!(f, "{}", s)
    }
}

/// Implementation of a rank on the chess board (horizontally from 0 to 7).
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd)]
pub struct Rank(pub u8);

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 1 + self.0)
    }
}

impl Square {
    /// Create a new [`Square`] instance from an unsigned char ([`u8`]).
    ///
    /// # Note
    /// Any value > 63 will be truncated to 63 and thus represent the h1 square.
    pub fn new(b: u8) -> Self {
        Square(b & 63)
    }

    /// Create a new [`Square`] instance from a [`usize`].
    ///
    /// # Note
    /// This simply casts the usize to a u8 and truncates any values > 63.
    /// Intended to be used for more efficient table lookups.
    pub fn from_index(i: usize) -> Self {
        Square((i as u8) & 63)
    }

    /// Get the file of the square as an unsigned char ([`u8`]).
    pub fn file_as_u8(&self) -> u8 {
        self.0 % 8
    }

    /// Get the rank of the square as an unsigned char ([`u8`]).
    pub fn rank_as_u8(&self) -> u8 {
        7 - (self.0 / 8)
    }

    /// Get the file of the square as a [`File`].
    pub fn file(&self) -> File {
        File(self.0 % 8)
    }

    /// Get the rank of the square as a [`Rank`].
    pub fn rank(&self) -> Rank {
        Rank(7 - (self.0 / 8))
    }

    /// Get the squares index value as a [`usize`].
    pub fn index(&self) -> usize {
        self.0 as usize
    }

    /// Create a new [`Square`] instance from a string.
    ///
    /// # Panics
    /// Iff the string was not a valid chess square.
    pub fn from_str(s: &str) -> Self {
        Square::try_from_str(s).unwrap()
    }

    /// Try to create a new [`Square`] instance from a string.
    ///
    /// # Errors
    /// Iff the string was not a valid chess square.
    pub fn try_from_str(s: &str) -> Result<Self> {
        let s: Square = match s.to_lowercase().as_str() {
            "a8" => Square(0),
            "b8" => Square(1),
            "c8" => Square(2),
            "d8" => Square(3),
            "e8" => Square(4),
            "f8" => Square(5),
            "g8" => Square(6),
            "h8" => Square(7),
            "a7" => Square(8),
            "b7" => Square(9),
            "c7" => Square(10),
            "d7" => Square(11),
            "e7" => Square(12),
            "f7" => Square(13),
            "g7" => Square(14),
            "h7" => Square(15),
            "a6" => Square(16),
            "b6" => Square(17),
            "c6" => Square(18),
            "d6" => Square(19),
            "e6" => Square(20),
            "f6" => Square(21),
            "g6" => Square(22),
            "h6" => Square(23),
            "a5" => Square(24),
            "b5" => Square(25),
            "c5" => Square(26),
            "e5" => Square(27),
            "d5" => Square(28),
            "f5" => Square(29),
            "g5" => Square(30),
            "h5" => Square(31),
            "a4" => Square(32),
            "b4" => Square(33),
            "c4" => Square(34),
            "d4" => Square(35),
            "e4" => Square(36),
            "f4" => Square(37),
            "g4" => Square(38),
            "h4" => Square(39),
            "a3" => Square(40),
            "b3" => Square(41),
            "c3" => Square(42),
            "d3" => Square(43),
            "e3" => Square(44),
            "f3" => Square(45),
            "g3" => Square(46),
            "h3" => Square(47),
            "a2" => Square(48),
            "b2" => Square(49),
            "c2" => Square(50),
            "d2" => Square(51),
            "e2" => Square(52),
            "f2" => Square(53),
            "g2" => Square(54),
            "h2" => Square(55),
            "a1" => Square(56),
            "b1" => Square(57),
            "c1" => Square(58),
            "d1" => Square(59),
            "e1" => Square(60),
            "f1" => Square(61),
            "g1" => Square(62),
            "h1" => Square(63),
            _ => {
                return Err(Box::new(ChessifyError::UnknownSquare(s.to_string())));
            }
        };

        Ok(s)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_ok() {
        let h1 = Square::from_str("h1");
        let c8 = Square::from_str("c8");
        let e4 = Square::from_str("E4");

        assert_eq!(Square(63), h1);
        assert_eq!(Square::new(2), c8);
        assert_eq!(Square::from_index(36 as usize), e4);
        assert_eq!(63 as usize, h1.index());
    }

    #[test]
    fn file_and_rank_ok() {
        let d7 = Square::from_str("d7");
        let f1 = Square::from_str("f1");

        assert_eq!(6u8, d7.rank_as_u8());
        assert_eq!(Rank(6), d7.rank());

        assert_eq!(5u8, f1.file_as_u8());
        assert_eq!(File(5), f1.file());

        let ds = d7.to_string();
        let fs = f1.to_string();
        assert_eq!('d', ds.chars().next().unwrap());
        assert_eq!('1', fs.chars().nth(1).unwrap());
    }

    #[test]
    #[should_panic]
    fn from_str_err() {
        Square::from_str("q4");
    }
}
