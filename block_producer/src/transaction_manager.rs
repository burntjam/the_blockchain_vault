use std::sync::Arc;
use config_lib::ChainConfig;
use super::transaction_processor::*;
use super::mock::{MockTransactionProcessor,MockTransactionProcessorFactory};

pub trait TransactionManager {
    fn process(&self,transaction: Vec<u8>);
}

pub struct BlockTransactionManager {
    processor_factory: Arc<dyn TransactionProcessorFactory>,
}


impl TransactionManager for BlockTransactionManager {
    fn process(&self, transaction: Vec<u8>) {
        self.processor_factory.createProcessor(transaction).process();
    }
}

impl BlockTransactionManager {
    pub fn new(processor_factory: Arc<dyn TransactionProcessorFactory>) -> Arc<dyn TransactionManager> {
        let config = ChainConfig::new().unwrap();
        Arc::new(BlockTransactionManager { processor_factory }) as Arc<dyn TransactionManager>
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use spool_client::*;
    use spool_errors::*;
    use std::sync::{Mutex, Arc,mpsc};

    #[test]
    fn test_block_transaction_manager_new() {
        let block_transaction_manager = BlockTransactionManager::new(MockTransactionProcessorFactory::new());
        let transaction = vec![1,2,3,4,5,6];
        block_transaction_manager.process(transaction);
    }

}