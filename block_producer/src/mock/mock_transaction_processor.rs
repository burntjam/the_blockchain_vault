use crate::transaction_processor::*;
use std::sync::Arc;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;




pub struct MockTransactionProcessor;

#[async_trait]
impl TransactionProcessor for MockTransactionProcessor {
    async fn process(&self) {
        
    }
}

impl MockTransactionProcessor {
    pub fn new(transaction: Vec<u8>) -> Arc<dyn TransactionProcessor> {
        Arc::new(MockTransactionProcessor { }) as Arc<dyn TransactionProcessor>
    }
}

pub struct MockTransactionProcessorFactory;

impl TransactionProcessorFactory for MockTransactionProcessorFactory {
    fn createProcessor(&self, transaction: Vec<u8>) -> Arc<dyn TransactionProcessor> {
        MockTransactionProcessor::new(transaction)
    }
}

impl MockTransactionProcessorFactory {
    pub fn new() -> Arc<dyn TransactionProcessorFactory> {
        Arc::new(MockTransactionProcessorFactory { }) as Arc<dyn TransactionProcessorFactory>
    }
}