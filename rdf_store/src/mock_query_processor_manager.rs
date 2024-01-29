use async_trait::async_trait;
use spool_client::*;
use config_lib::ChainConfig;
use std::sync::{Arc, Mutex};
use rdf_store_client::message::*;
use crate::DiskStoreManager;
use crate::query_processor::*;
use crate::mock_query_processor::*;
use crate::query_processor_manager::*;


pub struct MockQueryProcessManager;

impl MockQueryProcessManager {
    pub fn new() -> Result<Arc<dyn QueryProcessManager>,Box<dyn std::error::Error>> {
        Ok(Arc::new(MockQueryProcessManager {  }) as Arc<dyn QueryProcessManager>)
    }
}

impl QueryProcessManager for MockQueryProcessManager {
    fn createProcessor(&self,rdf_message: &RdfQueryMessage) -> Result<Arc<dyn QueryProcessor>,Box<dyn std::error::Error>> {
        Ok(MockQueryProcessor::new()?)
    }
}