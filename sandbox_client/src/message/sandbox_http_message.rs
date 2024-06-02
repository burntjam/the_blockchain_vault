use serde::{Deserialize, Serialize};
use derive_new::new;
use crate::SandboxMessageError;


// Define your message types using structs
#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct SandboxHttpMessage{
    pub contract: Vec<u8>,
}

// Define your message types using structs
#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct SandboxHttpResponseMessage{
    pub body: Vec<u8>,
}

pub fn deserialize_http_sandbox_message(json_message: &str) -> Result<SandboxHttpMessage,Box<dyn std::error::Error>> {
    let deserialized: SandboxHttpMessage = serde_json::from_str(json_message)?;
    Ok(deserialized)
}

pub fn serialize_http_sandbox_message(message: &SandboxHttpMessage) -> Result<String,Box<dyn std::error::Error>> {
    let json_message = serde_json::to_string(message)?;
    Ok(json_message)
}

pub fn deserialize_http_sandbox_bin_message(bin_message: &Vec<u8>) -> Result<SandboxHttpMessage,Box<dyn std::error::Error>> {
    match String::from_utf8(bin_message.clone()) {
        Ok(string) => deserialize_http_sandbox_message(&string),
        Err(e) => Err(Box::new(SandboxMessageError{message:"Failed failed to deserialize as the binary message is not a UTF8 string".to_string()})),
    }
}

pub fn serialize_http_sandbox_bin_message(message: &SandboxHttpMessage) -> Result<Vec<u8>,Box<dyn std::error::Error>> {
    let json_message = serialize_http_sandbox_message(message)?;
    Ok(json_message.into_bytes())
}

pub fn deserialize_http_sandbox_response_message(json_message: &str) -> Result<SandboxHttpResponseMessage,Box<dyn std::error::Error>> {
    let deserialized: SandboxHttpResponseMessage = serde_json::from_str(json_message)?;
    Ok(deserialized)
}

pub fn serialize_http_sandbox_response_message(message: &SandboxHttpResponseMessage) -> Result<String,Box<dyn std::error::Error>> {
    let json_message = serde_json::to_string(message)?;
    Ok(json_message)
}

pub fn deserialize_http_sandbox_reponse_bin_message(bin_message: &Vec<u8>) -> Result<SandboxHttpResponseMessage,Box<dyn std::error::Error>> {
    match String::from_utf8(bin_message.clone()) {
        Ok(string) => deserialize_http_sandbox_response_message(&string),
        Err(e) => Err(Box::new(SandboxMessageError{message:"Failed failed to deserialize as the binary message is not a UTF8 string".to_string()})),
    }
}

pub fn serialize_http_sandbox_response_bin_message(message: &SandboxHttpResponseMessage) -> Result<Vec<u8>,Box<dyn std::error::Error>> {
    let json_message = serialize_http_sandbox_response_message(message)?;
    Ok(json_message.into_bytes())
}
