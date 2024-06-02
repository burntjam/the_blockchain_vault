use peer2peer_protocol::{TransactionMessage,WebSocketMessage,serialize_bin_message};
use std::sync::{Arc,Mutex};
use peer2peer_protocol::BLOCK_PRODUCER_TRANSACTION;
use spool_client::SpoolConnectionManager;


pub trait TransactionProcessorClient: Sync + Send {
    fn submit_transaction(&self, bin_transaction: &Vec<u8>) -> Result<(),Box<dyn std::error::Error>>;
}

pub struct BlockTransactionProcessorClient {
    spool_manager: Arc<dyn SpoolConnectionManager>,
}


impl BlockTransactionProcessorClient {
    pub fn new(spool_manager: &Arc<dyn SpoolConnectionManager>) -> Result<Arc<dyn TransactionProcessorClient>,Box<dyn std::error::Error>> {
        Ok(Arc::new(BlockTransactionProcessorClient { spool_manager: spool_manager.clone() }))
    }
    async fn push(&self,bin_transaction: &Vec<u8>) -> Result<(),Box<dyn std::error::Error>>{
        let spool_client = self.spool_manager.create_connection(&String::from(BLOCK_PRODUCER_TRANSACTION)).await?;
        spool_client.push(bin_transaction.clone()).await.unwrap();
        Ok(())
    }
}

impl TransactionProcessorClient for BlockTransactionProcessorClient {
    fn submit_transaction(&self, bin_transaction: &Vec<u8>) -> Result<(),Box<dyn std::error::Error>> {
        let _ = futures::executor::block_on(self.push(bin_transaction))?;
        Ok(())
    }
}
