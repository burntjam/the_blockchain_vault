use std::sync::{Arc, Mutex};

use crate::{HttpRequest,TransactionRequest};


pub trait RequestManager: Sync + Send {
    fn create_http_request(&self) -> Result<Arc<dyn HttpRequest>, Box<dyn std::error::Error>>;
    fn create_transaction_request(&self, client_name: &String) -> Result<Arc<dyn TransactionRequest>, Box<dyn std::error::Error>>;
}