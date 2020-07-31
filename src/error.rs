use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ReversiError {
    pub message: String
}

impl ReversiError {
    pub fn new(message: impl Into<String>) -> ReversiError {
        ReversiError { message: message.into() }
    }
}

impl fmt::Display for ReversiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for ReversiError {}
