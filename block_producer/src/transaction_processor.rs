use std::sync::{Arc,Mutex};
use config_lib::ChainConfig;
use peer2peer_protocol::{deserialize_bin_message, WebSocketMessage, handler::message};
use rdf_lib::{StoreSessionFactory};
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;


#[async_trait]
pub trait TransactionProcessor {
    async fn process(&self);
}

pub struct BlockTransactionProcessor {
    transaction: Vec<u8>,
    session_factory: Arc<Mutex<dyn StoreSessionFactory>>,
}

#[async_trait]
impl TransactionProcessor for BlockTransactionProcessor {
    async fn process(&self) {
        let message = deserialize_bin_message(&self.transaction);
        if message.is_err() {
            return;
        }
        if let WebSocketMessage::Transaction(trans) = message.unwrap() {
            let account_id = trans.getAccountId();
            
        }
    }
}

impl BlockTransactionProcessor {
    pub fn new(transaction: Vec<u8>, session_factory: Arc<Mutex<dyn StoreSessionFactory>>) -> Arc<dyn TransactionProcessor> {
        let config = ChainConfig::new().unwrap();
        Arc::new(BlockTransactionProcessor { transaction,  session_factory}) as Arc<dyn TransactionProcessor>
    }
}

pub trait TransactionProcessorFactory : Sync + Send {
    fn createProcessor(&self, transaction: Vec<u8>) -> Arc<dyn TransactionProcessor>;
}

pub struct BlockTransactionProcessorFactory{
    session_factory: Arc<Mutex<dyn StoreSessionFactory>>,
}

impl TransactionProcessorFactory for BlockTransactionProcessorFactory {
    fn createProcessor(&self, transaction: Vec<u8>) -> Arc<dyn TransactionProcessor> {
        BlockTransactionProcessor::new(transaction, self.session_factory.clone())
    }
}

impl BlockTransactionProcessorFactory {
    pub fn new(session_factory: Arc<Mutex<dyn StoreSessionFactory>>) -> Arc<dyn TransactionProcessorFactory> {
        let config = ChainConfig::new().unwrap();
        Arc::new(BlockTransactionProcessorFactory { session_factory }) as Arc<dyn TransactionProcessorFactory>
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use spool_client::*;
    use std::error::Error;
    use std::sync::{Mutex, Arc,mpsc};
    use rdf_lib::MockStoreSessionFactory;


    #[test]
    fn test_block_processor_new() -> Result<(), Box<dyn Error>> {
        let transaction = vec![1,2,3,4,5,6];
        let transaction_processor = BlockTransactionProcessor::new(transaction, MockStoreSessionFactory::new()?);
        transaction_processor.process();
        Ok(())
    }

    #[test]
    fn test_block_transaction_processor_factory_new() -> Result<(), Box<dyn Error>> {
        let transaction = vec![1,2,3,4,5,6];
        BlockTransactionProcessorFactory::new(MockStoreSessionFactory::new()?).createProcessor(transaction).process();
        Ok(())
    }
}