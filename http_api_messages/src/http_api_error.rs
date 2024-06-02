use std::fmt;

#[derive(Debug)]
pub struct HttpApiError {
    pub message: String,
}

impl fmt::Display for HttpApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SandboxMessageError: {}", self.message)
    }
}

impl std::error::Error for HttpApiError {}