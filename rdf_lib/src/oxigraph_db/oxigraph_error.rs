use std::fmt;

#[derive(Debug)]
pub struct OxigraphStoreError {
    pub message: String,
}

impl fmt::Display for OxigraphStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OxigraphStoreError: {}", self.message)
    }
}

impl std::error::Error for OxigraphStoreError {}