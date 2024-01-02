use std::fmt;

#[derive(Debug)]
pub struct SpoolError {
    pub message: String,
}

impl fmt::Display for SpoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SpoolError: {}", self.message)
    }
}

impl std::error::Error for SpoolError {}


#[derive(Debug)]
pub struct SpoolDisconnectError {
    pub message: String,
}

impl fmt::Display for SpoolDisconnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SpoolDisconnectError: {}", self.message)
    }
}

impl std::error::Error for SpoolDisconnectError {}
