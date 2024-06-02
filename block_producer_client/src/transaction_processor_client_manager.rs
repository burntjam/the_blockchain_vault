use std::sync::Arc;
use config_lib::ChainConfig;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use spool_client::SpoolConnectionManager;
use crate::{TransactionProcessorClient,BlockTransactionProcessorClient};


pub trait TransactionProcessorClientManager: Sync + Send {
    fn create(&self) -> Result<Arc<dyn TransactionProcessorClient>,Box<dyn std::error::Error>>;
}

pub struct BlockTransactionProcessorClientManager {
    spool_manager: Arc<dyn SpoolConnectionManager>,
}


impl TransactionProcessorClientManager for BlockTransactionProcessorClientManager {
    fn create(&self) -> Result<Arc<dyn TransactionProcessorClient>,Box<dyn std::error::Error>> {
        Ok(BlockTransactionProcessorClient::new(&self.spool_manager)?)
    }
}

impl BlockTransactionProcessorClientManager {
    pub fn new(spool_manager: &Arc<dyn SpoolConnectionManager>) -> Arc<dyn TransactionProcessorClientManager> {
        let config = ChainConfig::new().unwrap();
        Arc::new(BlockTransactionProcessorClientManager { spool_manager: spool_manager.clone() })
    }
}