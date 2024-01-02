use serde::{Deserialize, Serialize};
use derive_new::new;
use super::network::*;

// Define your message types using structs
#[derive(Serialize, Deserialize, Debug,new, Clone)]
pub struct ElectionMessage{
    pub(crate) account_id: String,
    //signature: String,
}