use std::fmt;

#[derive(Debug)]
pub struct SandboxError {
    pub message: String,
}

impl fmt::Display for SandboxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SandboxError: {}", self.message)
    }
}

impl std::error::Error for SandboxError {}