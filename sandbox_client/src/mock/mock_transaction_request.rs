use async_trait::async_trait;
use crate::{TransactionRequest, SandboxMessage, SandboxResponseMessage};
use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use rdf_lib::store_result_set::*;
use spool_client::{SpoolConnectionManager,SpoolConnection};


pub struct MockTransactionRequest;

impl MockTransactionRequest {
    pub fn new() -> Result<Arc<dyn TransactionRequest>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(MockTransactionRequest {}) as Arc<dyn TransactionRequest>)
    }
}

#[async_trait]
impl TransactionRequest for MockTransactionRequest {
    async fn handle(&self, request: &SandboxMessage) -> Result<SandboxResponseMessage, Box<dyn std::error::Error>> {
        Ok(SandboxResponseMessage{contract:vec![],action_type: String::from("value"),change_set:vec![]})
    }
}