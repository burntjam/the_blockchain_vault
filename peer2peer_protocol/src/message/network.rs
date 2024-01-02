use serde::{Deserialize, Serialize};
use derive_new::new;
use chrono::{DateTime, Utc};

// Define your message types using structs
#[derive(Serialize, Deserialize, Debug,new, Clone)]
pub struct NetworkKeyQuery {
    pub(crate) account_id: String,
}

// Define your message types using structs
#[derive(Serialize, Deserialize, Debug,new, Clone)]
pub struct NetworkKeyResponse {
    pub(crate) account_id: String,
    pub(crate) keys: Vec<String>,
}


// Define your message types using structs
#[derive(Serialize, Deserialize, Debug,new, Clone)]
pub struct NetworkHeartBeat {
    pub cycle_id: String,
    pub timestamp: DateTime<Utc>,
    pub network_slot: u32,
    pub network_election_slot: u32,
    pub network_election_publish_slot: u32,
    pub network_confirmation_slot: u32,
}

