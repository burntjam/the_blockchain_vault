use std::fmt;

#[derive(Debug)]
pub struct ContractError {
    pub message: String,
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ContractError: {}", self.message)
    }
}

impl std::error::Error for ContractError {}
