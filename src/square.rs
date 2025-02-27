use crate::error::{ChessifyError, Result};

///
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Square(pub u8);

impl Square {
    ///
    pub fn from_index(i: usize) -> Self {
        Square(i as u8)
    }

    ///
    pub fn rank(&self) -> u8 {
        8 - self.0 / 8
    }

    ///
    pub fn file(&self) -> u8 {
        self.0 % 8
    }

    ///
    pub fn index(&self) -> u8 {
        self.0
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
