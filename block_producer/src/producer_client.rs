use peer2peer_protocol::{TransactionMessage,WebSocketMessage,serialize_bin_message};
use std::sync::{Arc,Mutex};
use crate::BLOCK_PRODUCER_TRANSACTION_SUBMISSION;
use spool_client::SpoolConnectionManager;

pub trait ProducerClient: Sync + Send {
    fn submit_transaction(&self, source_account_id: &String, target_account_id: &String, transaction_type: &String,
        transaction: &asn1_lib::TransactionMessage) -> Result<(),Box<dyn std::error::Error>>;
}

pub struct BlockProducerClient {
    spool_manager: Arc<dyn SpoolConnectionManager>,
}

impl BlockProducerClient {
    pub fn new(spool_manager: &Arc<dyn SpoolConnectionManager>) -> Result<Arc<Mutex<dyn ProducerClient>>,Box<dyn std::error::Error>> {
        Ok(Arc::new(Mutex::new(BlockProducerClient { spool_manager: spool_manager.clone() })))
    }
    async fn push(&self,transaction_message: &TransactionMessage) -> Result<(),Box<dyn std::error::Error>>{
        let spool_client = self.spool_manager.create_connection(&String::from(BLOCK_PRODUCER_TRANSACTION_SUBMISSION)).await?;
        spool_client.push(serialize_bin_message(&WebSocketMessage::Transaction(transaction_message.clone()))?).await.unwrap();
        Ok(())
    }
}

impl ProducerClient for BlockProducerClient {
    fn submit_transaction(&self, source_account_id: &String, target_account_id: &String, transaction_type: &String, 
        transaction: &asn1_lib::TransactionMessage) -> Result<(),Box<dyn std::error::Error>>{
        let binary_transaction = rasn::der::encode::<asn1_lib::TransactionMessage>(
            &transaction).unwrap();
        let transaction_message: TransactionMessage = peer2peer_protocol::TransactionMessage{
            source_account_id: source_account_id.clone(), target_account_id: target_account_id.clone(), 
            transaction_type: transaction_type.clone(), binary_transaction};
        let _ = futures::executor::block_on(self.push(&transaction_message))?;
        Ok(())
    }
}
