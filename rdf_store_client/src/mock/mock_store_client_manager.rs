use async_trait::async_trait;
use crate::{StoreClient,StoreClientManager,SpoolStoreClient, MockStoreClient};
use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use rdf_lib::store_result_set::*;
use std::cell::Cell;
use spool_client::SpoolConnectionManager;


pub struct MockStoreClientManager;

impl MockStoreClientManager {
    pub fn new() -> Result<Arc<Mutex<dyn StoreClientManager>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(Mutex::new(MockStoreClientManager {})) as Arc<Mutex<dyn StoreClientManager>>)
    }
}

impl StoreClientManager for MockStoreClientManager {
    fn create_client(&self) -> Result<Arc<Mutex<dyn StoreClient>>, Box<dyn std::error::Error>> {
        Ok(self.create_client_by_name(&String::from("MOCK"))?)
    }
    fn create_client_by_name(&self, client_name: &String) -> Result<Arc<Mutex<dyn StoreClient>>, Box<dyn std::error::Error>> {
        Ok(MockStoreClient::new()?)
    }
}