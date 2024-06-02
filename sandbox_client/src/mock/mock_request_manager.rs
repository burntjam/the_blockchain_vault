use async_trait::async_trait;
use crate::{TransactionRequest, HttpRequest, RequestManager, MockHttpRequest, MockTransactionRequest};
use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use rdf_lib::store_result_set::*;
use spool_client::{SpoolConnectionManager,SpoolConnection};

pub struct MockRequestManager;

impl MockRequestManager {
    pub fn new() -> Result<Arc<dyn RequestManager>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(MockRequestManager {}) as Arc<dyn RequestManager>)
    }
}

impl RequestManager for MockRequestManager {
    fn create_http_request(&self) -> Result<Arc<dyn HttpRequest>, Box<dyn std::error::Error>> {
            Ok(MockHttpRequest::new()?)
    }
    fn create_transaction_request(&self, client_name: &String) -> Result<Arc<dyn TransactionRequest>, Box<dyn std::error::Error>> {
        Ok(MockTransactionRequest::new()?)
    }
}