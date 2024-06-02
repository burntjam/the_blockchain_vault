use serde::{Deserialize, Serialize};
use derive_new::new;
use crate::{HttpApiError};
use std::collections::HashMap;



// Define your message types using structs
#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct HttpApiRequest{
    pub body: String,
    pub parameters: HashMap<String,String>,
    pub url: String,
}

pub fn deserialize_http_api_request(json_message: &str) -> Result<HttpApiRequest,Box<dyn std::error::Error>> {
    let deserialized: HttpApiRequest = serde_json::from_str(json_message)?;
    Ok(deserialized)
}

pub fn serialize_http_api_request(message: &HttpApiRequest) -> Result<String,Box<dyn std::error::Error>> {
    let json_message = serde_json::to_string(message)?;
    Ok(json_message)
}

pub fn deserialize_http_api_request_bin_message(bin_message: &Vec<u8>) -> Result<HttpApiRequest,Box<dyn std::error::Error>> {
    match String::from_utf8(bin_message.clone()) {
        Ok(string) => deserialize_http_api_request(&string),
        Err(e) => Err(Box::new(HttpApiError{message:"Failed failed to deserialize as the binary message is not a UTF8 string".to_string()})),
    }
}

pub fn serialize_http_api_request_bin_message(message: &HttpApiRequest) -> Result<Vec<u8>,Box<dyn std::error::Error>> {
    let json_message = serialize_http_api_request(message)?;
    Ok(json_message.into_bytes())
}