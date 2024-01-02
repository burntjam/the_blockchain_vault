use std::sync::Arc;
use config_lib::ChainConfig;
use spool_client::{SpoolConnection};
use super::mock::{MockTransactionManager};
use super::transaction_constants::*;
use super::transaction_manager::*;


pub trait TransactionConsumer {
    fn process(&self);
}

pub struct BlockTransactionConsumer {
    transaction_manager: Arc<dyn TransactionManager>,
    spool_client: Arc<dyn SpoolConnection>,
}

impl TransactionConsumer for BlockTransactionConsumer {
    fn process(&self) {
        while let result = self.spool_client.consumeFromTopic(&TRANSACTION_TOPIC_NAME.to_string()) {
            if result.is_err() {
                break;
            }
            let transactions = result.unwrap();
            transactions.iter().for_each(|entry| self.transaction_manager.process(entry.clone()) );
        }
    }
}

impl BlockTransactionConsumer {
    pub fn new(transaction_manager: Arc<dyn TransactionManager>, spool_client: Arc<dyn SpoolConnection>) -> Arc<dyn TransactionConsumer> {
        let config = ChainConfig::new().unwrap();
        Arc::new(BlockTransactionConsumer { transaction_manager, spool_client }) as Arc<dyn TransactionConsumer>
    }
}


#[cfg(test)]
mod tests {
    use crate::transaction_manager;

    use super::*;
    use spool_client::*;
    use spool_errors::*;
    use std::sync::{Mutex, Arc,mpsc};

    pub struct TransactionSpoolMockConnection {
        transactions: Arc<Mutex<Vec<Vec<u8>>>>,
    }

    impl spool_connection::SpoolConnection for TransactionSpoolMockConnection {
        fn push(&self,message: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
            
            Ok(())
        }

        fn pushToTopic(&self,message: Vec<u8>, topic: &String) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
        fn consume(&self) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
            let mut transactions = self.transactions.lock().unwrap();
            if transactions.len() > 0 {
                let result = transactions.clone();
                transactions.truncate(0);
                return Ok(result);
            }
            Err(Box::new(spool_errors::SpoolDisconnectError { message: "An error occurred".to_string() }))
        }

        fn consumeFromTopic(&self, topic: &String) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
            let mut transactions = self.transactions.lock().unwrap();
            if transactions.len() > 0 {
                let result = transactions.clone();
                transactions.truncate(0);
                return Ok(result);
            }
            Err(Box::new(spool_errors::SpoolDisconnectError { message: "An error occurred".to_string() }))
        }
    }


    pub fn createMockConnection() -> Result<Arc<dyn spool_connection::SpoolConnection>, Box<dyn std::error::Error>> {
        let entries = Arc::new(Mutex::new(vec![vec![1,2,3,4,5,6],vec![1,2,3,4,5,6],vec![1,2,3,4,5,6]]));
        Ok(Arc::new(TransactionSpoolMockConnection {transactions:entries}))
    }

    #[test]
    fn test_block_transaction_consumer_new() {
        let transaction_manager = MockTransactionManager::new();
        let spool_client = createMockConnection().unwrap();
        let blockTransactionConsumer = BlockTransactionConsumer::new(transaction_manager,spool_client);
        blockTransactionConsumer.process();
    }
}


