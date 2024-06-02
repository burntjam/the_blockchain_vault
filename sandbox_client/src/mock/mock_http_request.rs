use async_trait::async_trait;
use crate::{HttpRequest, SandboxHttpMessage, SandboxHttpResponseMessage};
use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use rdf_lib::store_result_set::*;
use spool_client::{SpoolConnectionManager,SpoolConnection};


pub struct MockHttpRequest;

impl MockHttpRequest {
    pub fn new() -> Result<Arc<dyn HttpRequest>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(MockHttpRequest {}) as Arc<dyn HttpRequest>)
    }
}


#[async_trait]
impl HttpRequest for MockHttpRequest {
    async fn handle(&self, request: &SandboxHttpMessage) -> Result<SandboxHttpResponseMessage, Box<dyn std::error::Error>> {
        Ok(SandboxHttpResponseMessage{body:vec![]})
    }
}