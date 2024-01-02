use std::sync::Arc;
use config_lib::ChainConfig;
use peer2peer_protocol::{deserialize_bin_message, WebSocketMessage, handler::message};

pub trait TransactionProcessor {
    fn process(&self);
}

pub struct BlockTransactionProcessor {
    transaction: Vec<u8>,
    rt: tokio::runtime::Runtime,
}

impl TransactionProcessor for BlockTransactionProcessor {
    fn process(&self) {
        self.rt.block_on(async {
            let message = deserialize_bin_message(&self.transaction);
            if message.is_err() {
                return;
            }
            if let WebSocketMessage::Transaction(trans) = message.unwrap() {
                let account_id = trans.getAccountId();
                
            }
        })
    }
}

impl BlockTransactionProcessor {
    pub fn new(transaction: Vec<u8>) -> Arc<dyn TransactionProcessor> {
        let config = ChainConfig::new().unwrap();
        let rt = tokio::runtime::Runtime::new().unwrap();
        Arc::new(BlockTransactionProcessor { transaction, rt}) as Arc<dyn TransactionProcessor>
    }
}

pub trait TransactionProcessorFactory {
    fn createProcessor(&self, transaction: Vec<u8>) -> Arc<dyn TransactionProcessor>;
}

pub struct BlockTransactionProcessorFactory{}

impl TransactionProcessorFactory for BlockTransactionProcessorFactory {
    fn createProcessor(&self, transaction: Vec<u8>) -> Arc<dyn TransactionProcessor> {
        BlockTransactionProcessor::new(transaction)
    }
}

impl BlockTransactionProcessorFactory {
    pub fn new() -> Arc<dyn TransactionProcessorFactory> {
        let config = ChainConfig::new().unwrap();
        Arc::new(BlockTransactionProcessorFactory { }) as Arc<dyn TransactionProcessorFactory>
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use spool_client::*;
    use spool_errors::*;
    use std::sync::{Mutex, Arc,mpsc};


    #[test]
    fn test_block_processor_new() {
        let transaction = vec![1,2,3,4,5,6];
        let transaction_processor = BlockTransactionProcessor::new(transaction);
        transaction_processor.process();
    }

    #[test]
    fn test_block_transaction_processor_factory_new() {
        let transaction = vec![1,2,3,4,5,6];
        BlockTransactionProcessorFactory::new().createProcessor(transaction).process();
    }
}