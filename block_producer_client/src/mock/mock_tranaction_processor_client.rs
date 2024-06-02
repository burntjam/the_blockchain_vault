use crate::transaction_processor_client::TransactionProcessorClient;
use std::sync::Arc;

pub struct MockTransactionProcessorClient{}

impl TransactionProcessorClient for MockTransactionProcessorClient {
    fn submit_transaction(&self, bin_transaction: &Vec<u8>) -> Result<(),Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl MockTransactionProcessorClient {
    pub fn new() -> Result<Arc<dyn TransactionProcessorClient>,Box<dyn std::error::Error>> {
        Ok(Arc::new(MockTransactionProcessorClient { }))
    }
}