use peer2peer_protocol::{TransactionMessage,WebSocketMessage,serialize_bin_message};
use std::sync::{Arc,Mutex};
use crate::{ProducerClient,BLOCK_PRODUCER_TRANSACTION_SUBMISSION};
use spool_client::SpoolConnectionManager;


pub struct MockProducerClient;

impl MockProducerClient {
    pub fn new() -> Result<Arc<Mutex<dyn ProducerClient>>,Box<dyn std::error::Error>> {
        Ok(Arc::new(Mutex::new(MockProducerClient { })))
    }
}

impl ProducerClient for MockProducerClient {
    fn submit_transaction(&self, source_account_id: &String, target_account_id: &String, transaction_type: &String, 
        transaction: &asn1_lib::TransactionMessage) -> Result<(),Box<dyn std::error::Error>>{
        Ok(())
    }
}