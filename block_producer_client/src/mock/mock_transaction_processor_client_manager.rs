use std::sync::Arc;
use crate::transaction_processor_client_manager::TransactionProcessorClientManager;
use crate::transaction_processor_client::TransactionProcessorClient;
use crate::mock_tranaction_processor_client::MockTransactionProcessorClient;

pub struct MockTransactionProcessorClientManager{}

impl TransactionProcessorClientManager for MockTransactionProcessorClientManager {
    fn create(&self) -> Result<Arc<dyn TransactionProcessorClient>,Box<dyn std::error::Error>> {
        Ok(MockTransactionProcessorClient::new()?)
    }
}

impl MockTransactionProcessorClientManager {
    pub fn new() -> Arc<dyn TransactionProcessorClientManager> {
        Arc::new(MockTransactionProcessorClientManager {})
    }
}