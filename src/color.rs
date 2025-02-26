use crate::error::{ChessifyError, Result};

/// Exhaustive enum of the all available colors.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Color {
    White,
    Black,
}

/// The number of different colors in the game of chess.
pub const NUM_COLORS: usize = 2;
/// An [`array`] containing the colors with placement corresponding to their respective index.
pub const COLORS: [Color; NUM_COLORS] = [Color::White, Color::Black];

impl Color {
    /// Get the [`usize`] index of the color.
    ///
    /// This is usually used for efficient table lookups.
    pub fn as_index(&self) -> usize {
        *self as usize
    }

    /// Create a new [`Color`] given a string.
    ///
    /// # Errors
    /// If either the string is empty or the string does not contain any of ('w', 'W', 'b', 'B')
    /// as the first character.
    pub fn from_str(s: &str) -> Result<Color> {
        match s.chars().next() {
            Some(c) => match c {
                'w' | 'W' => Ok(Color::White),
                'b' | 'B' => Ok(Color::Black),
                _ => Err(Box::new(ChessifyError::UnknownColor(s.to_string()))),
            },
            None => Err(Box::new(ChessifyError::UnknownColor(s.to_string()))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_index() {
        assert_eq!(0, Color::White.as_index());
        assert_eq!(1, Color::Black.as_index());
    }

    #[test]
    fn index_lookup() {
        assert_eq!(Color::White, COLORS[0]);
        assert_eq!(Color::Black, COLORS[Color::Black.as_index()]);
    }
}
