use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use crate::{ProducerClient,BlockProducerClient};
use spool_client::SpoolConnectionManager;

pub trait ProducerClientManager: Sync + Send {
    fn create_producer_client(&self) -> Result<Arc<Mutex<dyn ProducerClient>>,Box<dyn std::error::Error>>;
}


pub struct BlockProducerClientManager {
    spool_manager: Arc<dyn SpoolConnectionManager>,
}

impl BlockProducerClientManager {
    pub fn new(spool_manager: &Arc<dyn SpoolConnectionManager>) -> Result<Arc<Mutex<dyn ProducerClientManager>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new().unwrap();
        Ok(Arc::new(Mutex::new(BlockProducerClientManager { spool_manager: spool_manager.clone() })))
    }
}

impl ProducerClientManager for BlockProducerClientManager {
    fn create_producer_client(&self) -> Result<Arc<Mutex<dyn ProducerClient>>,Box<dyn std::error::Error>>{
        Ok(BlockProducerClient::new(&self.spool_manager)?)
    }
}