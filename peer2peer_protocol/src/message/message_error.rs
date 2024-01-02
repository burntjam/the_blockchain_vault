use std::fmt;

#[derive(Debug)]
pub struct MessageError {
    pub message: String,
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MessageError: {}", self.message)
    }
}

impl std::error::Error for MessageError {}