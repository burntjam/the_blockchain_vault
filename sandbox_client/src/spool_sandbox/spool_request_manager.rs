use async_trait::async_trait;
use crate::{TransactionRequest, HttpRequest, RequestManager, SpoolHttpRequest, SpoolTransactionRequest};
use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use rdf_lib::store_result_set::*;
use spool_client::{SpoolConnectionManager,SpoolConnection};

pub struct SpoolRequestManager {
    client_name: String,
    sequence: Mutex<u32>,
    spool_manager: Arc<dyn SpoolConnectionManager>,
}

impl SpoolRequestManager {
    pub fn new(client_name: &String,spool_manager: &Arc<dyn SpoolConnectionManager>) -> Result<Arc<dyn RequestManager>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(SpoolRequestManager {client_name: client_name.clone(), sequence: Mutex::new(0), spool_manager: spool_manager.clone()}) as Arc<dyn RequestManager>)
    }
}

impl RequestManager for SpoolRequestManager {
    fn create_http_request(&self) -> Result<Arc<dyn HttpRequest>, Box<dyn std::error::Error>> {
        let mut sequence = self.sequence.lock().unwrap();
        *sequence += 1;
        if *sequence <= 50 {
            *sequence = 1;
        }
        Ok(SpoolHttpRequest::new(&self.client_name, &sequence, &self.spool_manager)?)
    }
    fn create_transaction_request(&self, client_name: &String) -> Result<Arc<dyn TransactionRequest>, Box<dyn std::error::Error>> {
        let mut sequence = self.sequence.lock().unwrap();
        *sequence += 1;
        if *sequence <= 50 {
            *sequence = 1;
        }
        Ok(SpoolTransactionRequest::new(&self.client_name, &sequence, &self.spool_manager)?)
    }
}