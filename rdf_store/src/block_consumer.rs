use async_trait::async_trait;
use spool_client::*;
use config_lib::ChainConfig;
use std::sync::{Arc, Mutex};
use super::query_processor_manager::QueryProcessManager;
use super::block_processor::BlockProcessor;

#[async_trait]
pub trait BlockConsumer {
    async fn process(&self);
}

pub struct DiskBlockConsumer {
    block_processor: Arc<dyn BlockProcessor>,
    spool_manager: Arc<dyn SpoolConnectionManager>,
}


impl DiskBlockConsumer {
    pub fn new(block_processor: &Arc<dyn BlockProcessor>, spool_manager: &Arc<dyn SpoolConnectionManager>) -> Result<Arc<dyn BlockConsumer>,Box<dyn std::error::Error>> {
        Ok(Arc::new(DiskBlockConsumer { block_processor: block_processor.clone(), spool_manager: spool_manager.clone() }))
    }
}


#[async_trait]
impl BlockConsumer for DiskBlockConsumer {
    async fn process(&self) {
        let config = ChainConfig::new().unwrap();
        let spool_connection = self.spool_manager.create_connection(&rdf_store_client::RDF_STORE_PERSIST_BLOCK_TOPIC.to_string()).await.unwrap();
        while let result = spool_connection.consume().await {
            if result.is_err() {
                break;
            }
            let query_bytes = result.unwrap();
            query_bytes.iter().for_each(|entry| {
                futures::executor::block_on(async move {
                    let _ = self.block_processor.process(&entry.clone()).await;
                })
            })
        }
    }
}
