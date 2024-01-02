use std::fmt;

#[derive(Debug)]
struct TransactionError {
    message: String,
}

impl fmt::Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyError: {}", self.message)
    }
}

impl std::error::Error for TransactionError {}
