use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use crate::{ProducerClient,MockProducerClient,ProducerClientManager};
use spool_client::SpoolConnectionManager;


pub struct MockProducerClientManager;

impl MockProducerClientManager {
    pub fn new() -> Result<Arc<Mutex<dyn ProducerClientManager>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new().unwrap();
        Ok(Arc::new(Mutex::new(MockProducerClientManager { })))
    }
}

impl ProducerClientManager for MockProducerClientManager {
    fn create_producer_client(&self) -> Result<Arc<Mutex<dyn ProducerClient>>,Box<dyn std::error::Error>>{
        Ok(MockProducerClient::new()?)
    }
}