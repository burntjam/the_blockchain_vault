use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fmt;
use rdf_lib::store_result_set::*;

// Define your message types using structs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RdfQueryMessage{
    pub client: String,
    pub client_id: String,
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RdfResponseMessage{
    pub client: String,
    pub client_id: String,
    pub rdf_result_set: RdfResultSet,
}


// Define your message types using structs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RdfSignedBlock{
    pub signed_block: Vec<u8>,
}

impl RdfSignedBlock {
    pub fn new(signed_block: &Vec<u8>) -> Result<RdfSignedBlock,Box<dyn std::error::Error>> {
        Ok(RdfSignedBlock{signed_block: signed_block.clone()})
    }
}