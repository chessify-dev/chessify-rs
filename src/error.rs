use std::error;
use std::fmt;
use std::result;

///
pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub enum Error {
    InvalidFen(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidFen(s) => write!(f, "{}", s),
        }
    }
}

impl error::Error for Error {}

