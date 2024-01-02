use std::sync::{Arc, Mutex};
use crate::StoreState;


pub trait StoreManager {

    fn createOrRetrieveState(&self) -> Result<Arc<Mutex<dyn StoreState>>, Box<dyn std::error::Error>>;
    
}