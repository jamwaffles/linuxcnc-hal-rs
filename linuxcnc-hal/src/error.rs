use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PointerError {
    Null,
}

impl fmt::Display for PointerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "Pointer address is null"),
        }
    }
}

impl Error for PointerError {}
