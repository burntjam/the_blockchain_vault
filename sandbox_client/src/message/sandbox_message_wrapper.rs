use serde::{Deserialize, Serialize};
use derive_new::new;
use crate::{SandboxMessage,SandboxMessageError};


// Define your message types using structs
#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct SandboxMessageWrapper{
    pub client: String,
    pub client_id: u32,
    pub sandbox_message: SandboxMessage,
}

pub fn deserialize_sandbox_message_wrapper(json_message: &str) -> Result<SandboxMessageWrapper,Box<dyn std::error::Error>> {
    let deserialized: SandboxMessageWrapper = serde_json::from_str(json_message)?;
    Ok(deserialized)
}

pub fn serialize_sandbox_message_wrapper(message: &SandboxMessageWrapper) -> Result<String,Box<dyn std::error::Error>> {
    let json_message = serde_json::to_string(message)?;
    Ok(json_message)
}

pub fn deserialize_sandbox_message_wrapper_bin(bin_message: &Vec<u8>) -> Result<SandboxMessageWrapper,Box<dyn std::error::Error>> {
    match String::from_utf8(bin_message.clone()) {
        Ok(string) => deserialize_sandbox_message_wrapper(&string),
        Err(e) => Err(Box::new(SandboxMessageError{message:"Failed failed to deserialize as the binary message is not a UTF8 string".to_string()})),
    }
}

