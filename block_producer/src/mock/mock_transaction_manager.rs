use crate::TransactionManager;
use std::sync::Arc;


pub struct MockTransactionManager;


impl TransactionManager for MockTransactionManager {
    fn process(&self, transaction: &Vec<u8>) {
        
    }
}

impl MockTransactionManager {
    pub fn new() -> Arc<dyn TransactionManager> {
        Arc::new(MockTransactionManager { }) as Arc<dyn TransactionManager>
    }
}