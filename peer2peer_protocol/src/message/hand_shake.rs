use serde::{Deserialize, Serialize};
use derive_new::new;
use super::network::*;

// Define your message types using structs
#[derive(Serialize, Deserialize, Debug,new, Clone)]
pub struct HelloMessage{
    pub(crate) account_id: String,
    //signature: String,
}


#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct HelloResponse{
    pub(crate) account_id: String,
    pub(crate) seed_hash: String,

}

#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct ChallengeMessage{
    pub(crate) account_id: String,
    pub(crate) seed_hash: String,
    pub(crate) challenge: Vec<String>,
    pub(crate) signature: String,
}

#[derive(Serialize, Deserialize, Debug, new, Clone)]
pub struct ChallengeResponse{
    pub(crate) account_id: String,
    pub(crate) status: String,
    pub(crate) peers: Vec<String>,
    pub(crate) network_key_response: Option<NetworkKeyResponse>,
}


