use std::sync::{Arc,Mutex};
use config_lib::ChainConfig;
use peer2peer_protocol::server::transaction;
use peer2peer_protocol::{deserialize_bin_message, WebSocketMessage, handler::message, TransactionMessage};
use rdf_lib::{StoreSessionFactory};
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use crate::tangle_manager::TangleManager;
use asn1_lib::{Transaction,TransactionWrapper, SignedTransaction, SignedChangeSet, ChangeSet};



#[async_trait]
pub trait TransactionProcessor {
    async fn process(&self);
}

pub struct BlockTransactionProcessor {
    transaction: Vec<u8>,
    session_factory: Arc<Mutex<dyn StoreSessionFactory>>,
    tangle_manager: Arc<Mutex<dyn TangleManager>>,
}

impl BlockTransactionProcessor {
    pub fn new(transaction: Vec<u8>, session_factory: Arc<Mutex<dyn StoreSessionFactory>>, tangle_manager: Arc<Mutex<dyn TangleManager>>) -> Arc<dyn TransactionProcessor> {
        let config = ChainConfig::new().unwrap();
        Arc::new(BlockTransactionProcessor { transaction,  session_factory, tangle_manager}) as Arc<dyn TransactionProcessor>
    }
    pub fn processTransactionMessage(&self, transactionMessage: &TransactionMessage) -> Result<(), Box<dyn std::error::Error>> {
        // check the account from the tangle manager
        let tangle_manager_ref = self.tangle_manager.lock().unwrap();

        if !tangle_manager_ref.managed_transaction(transactionMessage)? {
            // route out
            return Ok(());
        }
        
        let account_id = transactionMessage.getAccountId().clone();
        
        let mut asnTransactionMessage = rasn::der::decode::<asn1_lib::TransactionMessage>(
            &transactionMessage.binary_transaction.clone()).unwrap();
        let _ = self.processAsnTransactionMessage(&transactionMessage.transaction_type, &mut asnTransactionMessage);
        Ok(())
    }
    fn processAsnTransactionMessage(&self, transaction_state: &String, transactionMessage: &mut asn1_lib::TransactionMessage) -> Result<(bool), Box<dyn std::error::Error>> {
        transactionMessage.sideTransactions.iter_mut().all(|transaction: &mut asn1_lib::TransactionMessage|
            self.processAsnTransactionMessage(transaction_state, transaction).unwrap()
        );
        
        let signedTransaction = rasn::der::decode::<asn1_lib::SignedTransaction>(
            &transactionMessage.transaction.signature.clone()).unwrap();
        let changeSets: &mut Vec<SignedChangeSet> = &mut transactionMessage.transaction.changeSet;
        
        signedTransaction.transaction.actions.iter().all(|action|
            self.processAction(transaction_state, &signedTransaction, action, changeSets).unwrap()
        );

        Ok(true)
    }
    fn processAction(&self, transaction_state: &String, signedTransaction: &asn1_lib::SignedTransaction, action: &asn1_lib::Action, changeSets: &mut Vec<SignedChangeSet>) 
        -> Result<(bool), Box<dyn std::error::Error>> {

        let mut change_set = asn1_lib::ChangeSet{
            version: 1,
            transactionHash: signedTransaction.transactionHash.clone(),
            accountHash: (if transaction_state == "CREDIT" {
                signedTransaction.transaction.targetAccount.clone()}else{
                    signedTransaction.transaction.targetAccount.clone()}),
            status: (if transaction_state == "CREDIT" {
                asn1_lib::Status::credit}else{
                    asn1_lib::Status::debit}),
            changes: Vec::new(),
        };

        

        let signed_change_set = SignedChangeSet{
            changeSet: change_set,
            changeSetHash: rasn::types::OctetString::new(),
            signature: rasn::types::OctetString::new(),
        };
        changeSets.push(signed_change_set);
        Ok(true)
    }
}


#[async_trait]
impl TransactionProcessor for BlockTransactionProcessor {
    async fn process(&self) {
        let message = deserialize_bin_message(&self.transaction);
        if message.is_err() {
            return;
        }
        if let WebSocketMessage::Transaction(trans) = message.unwrap() {
            self.processTransactionMessage(&trans);
        }
    }
}


pub trait TransactionProcessorFactory : Sync + Send {
    fn createProcessor(&self, transaction: Vec<u8>) -> Arc<dyn TransactionProcessor>;
}

pub struct BlockTransactionProcessorFactory{
    session_factory: Arc<Mutex<dyn StoreSessionFactory>>,
    tangle_manager: Arc<Mutex<dyn TangleManager>>,
}

impl TransactionProcessorFactory for BlockTransactionProcessorFactory {
    fn createProcessor(&self, transaction: Vec<u8>) -> Arc<dyn TransactionProcessor> {
        BlockTransactionProcessor::new(transaction, self.session_factory.clone(), self.tangle_manager.clone())
    }
}

impl BlockTransactionProcessorFactory {
    pub fn new(session_factory: Arc<Mutex<dyn StoreSessionFactory>>, tangle_manager: Arc<Mutex<dyn TangleManager>>) -> Arc<dyn TransactionProcessorFactory> {
        let config = ChainConfig::new().unwrap();
        Arc::new(BlockTransactionProcessorFactory { session_factory, tangle_manager }) as Arc<dyn TransactionProcessorFactory>
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use spool_client::*;
    use std::error::Error;
    use std::sync::{Mutex, Arc,mpsc};
    use rdf_lib::MockStoreSessionFactory;
    use crate::mock::MockTangleManager;


    #[test]
    fn test_block_processor_new() -> Result<(), Box<dyn Error>> {
        let transaction = vec![1,2,3,4,5,6];
        let transaction_processor = BlockTransactionProcessor::new(
            transaction, MockStoreSessionFactory::new()?, MockTangleManager::new()?);
        transaction_processor.process();
        Ok(())
    }

    #[test]
    fn test_block_transaction_processor_factory_new() -> Result<(), Box<dyn Error>> {
        let transaction = vec![1,2,3,4,5,6];
        BlockTransactionProcessorFactory::new(MockStoreSessionFactory::new()?, MockTangleManager::new()?)
            .createProcessor(transaction).process();
        Ok(())
    }
}