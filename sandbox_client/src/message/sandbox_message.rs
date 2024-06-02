use serde::{Deserialize, Serialize};
use derive_new::new;
use crate::SandboxMessageError;

// Define your message types using structs
#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct SandboxMessage{
    pub contract: Vec<u8>,
    pub action_type: String,
    pub change_set: Vec<u8>,
}


// Define your message types using structs
#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct SandboxResponseMessage{
    pub contract: Vec<u8>,
    pub action_type: String,
    pub change_set: Vec<u8>,
}


pub fn deserialize_sandbox_message(json_message: &str) -> Result<SandboxMessage,Box<dyn std::error::Error>> {
    let deserialized: SandboxMessage = serde_json::from_str(json_message)?;
    Ok(deserialized)
}

pub fn serialize_sandbox_message(message: &SandboxMessage) -> Result<String,Box<dyn std::error::Error>> {
    let json_message = serde_json::to_string(message)?;
    Ok(json_message)
}

pub fn deserialize_sandbox_bin_message(bin_message: &Vec<u8>) -> Result<SandboxMessage,Box<dyn std::error::Error>> {
    match String::from_utf8(bin_message.clone()) {
        Ok(string) => deserialize_sandbox_message(&string),
        Err(e) => Err(Box::new(SandboxMessageError{message:"Failed failed to deserialize as the binary message is not a UTF8 string".to_string()})),
    }
}

pub fn serialize_http_sandbox_bin_message(message: &SandboxMessage) -> Result<Vec<u8>,Box<dyn std::error::Error>> {
    let json_message = serialize_sandbox_message(message)?;
    Ok(json_message.into_bytes())
}

pub fn deserialize_sandbox_response_message(json_message: &str) -> Result<SandboxResponseMessage,Box<dyn std::error::Error>> {
    let deserialized: SandboxResponseMessage = serde_json::from_str(json_message)?;
    Ok(deserialized)
}

pub fn serialize_sandbox_response_message(message: &SandboxResponseMessage) -> Result<String,Box<dyn std::error::Error>> {
    let json_message = serde_json::to_string(message)?;
    Ok(json_message)
}

pub fn deserialize_sandbox_reponse_bin_message(bin_message: &Vec<u8>) -> Result<SandboxResponseMessage,Box<dyn std::error::Error>> {
    match String::from_utf8(bin_message.clone()) {
        Ok(string) => deserialize_sandbox_response_message(&string),
        Err(e) => Err(Box::new(SandboxMessageError{message:"Failed failed to deserialize as the binary message is not a UTF8 string".to_string()})),
    }
}


