use async_trait::async_trait;
use spool_client::*;
use config_lib::ChainConfig;
use std::sync::{Arc, Mutex};
use rdf_store_client::message::*;
use crate::DiskStoreManager;
use crate::query_processor::*;


pub struct MockQueryProcessor;

impl MockQueryProcessor {
    pub fn new() -> Result<Arc<dyn QueryProcessor>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new().unwrap();
        Ok(Arc::new(MockQueryProcessor { }) as Arc<dyn QueryProcessor>)
    }
}

#[async_trait]
impl QueryProcessor for MockQueryProcessor {
    async fn process(&self) -> Result<(),Box<dyn std::error::Error>> {
        Ok(())
    }
}