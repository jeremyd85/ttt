use std::any::TypeId;
use std::fmt;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum TicTacToeError {
    InvalidBoard,
    InvalidPlayerEnum,
    IllegalMove,
    OutOfBounds
    
}

impl Display for TicTacToeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::IllegalMove => write!(f, "illegal move - must be empty spot"),
            Self::InvalidPlayerEnum => 
                write!(f, "invalid PlayerEnum - convert from &str (\" \",  \"X\", \"O\") or u32 (0, 1, 2)"),
            Self::InvalidBoard => write!(f, "invalid Board"),
            Self::OutOfBounds => write!(f, "attempted to access Board position not in range (0-8) inclusive")
        }
    }
}

impl Error for TicTacToeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

