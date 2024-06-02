use serde::{Deserialize, Serialize};
use derive_new::new;
use crate::{SandboxHttpMessage,SandboxMessageError};


// Define your message types using structs
#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct SandboxHttpMessageWrapper{
    pub client: String,
    pub client_id: u32,
    pub sandbox_http_message: SandboxHttpMessage,
}

pub fn deserialize_http_sandbox_message_wrapper(json_message: &str) -> Result<SandboxHttpMessageWrapper,Box<dyn std::error::Error>> {
    let deserialized: SandboxHttpMessageWrapper = serde_json::from_str(json_message)?;
    Ok(deserialized)
}

pub fn serialize_http_sandbox_message_wrapper(message: &SandboxHttpMessageWrapper) -> Result<String,Box<dyn std::error::Error>> {
    let json_message = serde_json::to_string(message)?;
    Ok(json_message)
}

pub fn deserialize_sandbox_http_message_wrapper_bin(bin_message: &Vec<u8>) -> Result<SandboxHttpMessageWrapper,Box<dyn std::error::Error>> {
    match String::from_utf8(bin_message.clone()) {
        Ok(string) => deserialize_http_sandbox_message_wrapper(&string),
        Err(e) => Err(Box::new(SandboxMessageError{message:"Failed failed to deserialize as the binary message is not a UTF8 string".to_string()})),
    }
}