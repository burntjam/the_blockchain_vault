use std::fmt;

#[derive(Debug)]
pub struct SandboxMessageError {
    pub message: String,
}

impl fmt::Display for SandboxMessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SandboxMessageError: {}", self.message)
    }
}

impl std::error::Error for SandboxMessageError {}