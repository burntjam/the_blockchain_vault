use std::fmt;

#[derive(Debug)]
pub struct TransactionError {
    pub message: String,
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TransactionError: {}", self.message)
    }
}

impl std::error::Error for TransactionError {}
