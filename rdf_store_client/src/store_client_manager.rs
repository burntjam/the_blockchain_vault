use std::sync::{Arc, Mutex};

use crate::store_client::*;


pub trait StoreClientManager: Sync + Send {
    fn create_client(&self) -> Result<Arc<Mutex<dyn StoreClient>>, Box<dyn std::error::Error>>;
    fn create_client_by_name(&self, client_name: &String) -> Result<Arc<Mutex<dyn StoreClient>>, Box<dyn std::error::Error>>;
}