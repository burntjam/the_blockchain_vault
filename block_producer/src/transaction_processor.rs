use std::io::Read;
use std::sync::{Arc,Mutex};
use std::thread::sleep;
use config_lib::ChainConfig;
use peer2peer_protocol::server::transaction;
use peer2peer_protocol::{deserialize_bin_message,serialize_bin_message, WebSocketMessage, handler::message, TransactionMessage};
use rdf_lib::{StoreSessionFactory};
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use crate::tangle_manager::TangleManager;
use crate::{action_executor, producer_client_manager, ActionExecutor, ActionManager, Contract, ContractManager, ProducerClientManager, TransactionError};
use asn1_lib::{ChangeSet, SignedChangeSet, SignedTransaction, Status, Transaction, TransactionWrapper};
use block_producer_client::transaction_processor_client::TransactionProcessorClient;
use block_producer_client::transaction_processor_client_manager::TransactionProcessorClientManager;


fn parse_octet_string(source: &rasn::types::OctetString) -> Result<String, Box<dyn std::error::Error>> {
    std::str::from_utf8(source).map(|s|s.to_string()).
        map_err(|e| Box::new(TransactionError{message:String::from("")}) as Box<dyn std::error::Error>)
}


#[async_trait]
pub trait TransactionProcessor: Sync + Send {
    async fn process(&self) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct BlockTransactionProcessor {
    transaction: Vec<u8>,
    session_factory: Arc<Mutex<dyn StoreSessionFactory>>,
    tangle_manager: Arc<Mutex<dyn TangleManager>>,
    contract_manager: Arc<Mutex<dyn ContractManager>>,
    action_manager: Arc<Mutex<dyn ActionManager>>,
    producer_client_manager: Arc<Mutex<dyn ProducerClientManager>>,
    transaction_processor_client_manager: Arc<dyn TransactionProcessorClientManager>,
}

impl BlockTransactionProcessor {
    pub fn new(transaction: &Vec<u8>, session_factory: &Arc<Mutex<dyn StoreSessionFactory>>, 
        tangle_manager: &Arc<Mutex<dyn TangleManager>>, contract_manager: &Arc<Mutex<dyn ContractManager>>,
        action_manager: &Arc<Mutex<dyn ActionManager>>, producer_client_manager: &Arc<Mutex<dyn ProducerClientManager>>,
        transaction_processor_client_manager: &Arc<dyn TransactionProcessorClientManager>) -> Arc<dyn TransactionProcessor> {
        let config = ChainConfig::new().unwrap();
        Arc::new(BlockTransactionProcessor { transaction: transaction.clone(),  session_factory: session_factory.clone(), 
            tangle_manager: tangle_manager.clone(), contract_manager: contract_manager.clone(),
            action_manager: action_manager.clone(), producer_client_manager: producer_client_manager.clone(),
            transaction_processor_client_manager: transaction_processor_client_manager.clone()}) as Arc<dyn TransactionProcessor>
    }
    pub fn processTransactionMessage(&self, transactionMessage: &TransactionMessage) -> Result<(), Box<dyn std::error::Error>> {
        // check the account from the tangle manager
        let tangle_manager_ref = self.tangle_manager.lock().unwrap();
        println!("Testing step 1");
        if !tangle_manager_ref.managed_transaction(transactionMessage)? {
            // route out
            return Ok(());
        }
        println!("Testing step 2");
        let account_id = transactionMessage.getAccountId().clone();
        println!("Testing step 3");
        let mut asnTransactionMessage = rasn::der::decode::<asn1_lib::TransactionMessage>(
            &transactionMessage.binary_transaction.clone()).unwrap();
        println!("Testing step 4 : {}", asnTransactionMessage.transaction.signedTransaction.transaction.actions.len());
        let _ = self.processAsnTransactionMessage(&transactionMessage.transaction_type, &mut asnTransactionMessage)?;
        let producer_client_manager = self.producer_client_manager.lock().unwrap();
        let producer_client = producer_client_manager.create_producer_client()?;
        let producer_client_ref = producer_client.lock().unwrap();
        let _ = producer_client_ref.submit_transaction(&transactionMessage.source_account_id, 
            &transactionMessage.target_account_id, &transactionMessage.transaction_type, &asnTransactionMessage)?;

        self.incrementTransaction(&mut asnTransactionMessage)?;
        
        Ok(())
    }
    fn processAsnTransactionMessage(&self, transaction_state: &String, transactionMessage: &mut asn1_lib::TransactionMessage) -> Result<(bool), Box<dyn std::error::Error>> {
        println!("Before a run");
        transactionMessage.sideTransactions.iter_mut().all(|transaction: &mut asn1_lib::TransactionMessage| {
            println!("Looping through");
            self.processAsnTransactionMessage(transaction_state, transaction).unwrap()
        });
        
        let signedTransaction = 
            transactionMessage.transaction.signedTransaction.clone();
        let changeSets: &mut Vec<SignedChangeSet> = &mut transactionMessage.transaction.changeSet;
        
        println!("Loop through the actions : {}",signedTransaction.transaction.actions.len());
        signedTransaction.transaction.actions.iter().all(|action| {
            println!("Process action");
            self.processAction(transaction_state, &signedTransaction, action, changeSets).unwrap()
        });
        println!("Return the result");
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

        let contract_manager_ref = self.contract_manager.lock().unwrap();
        let contract: Arc<Mutex<dyn Contract>> = if let Some(value) = action.contract.clone() {
            contract_manager_ref.get_contract(&value.to_vec())?
        } else if let Some(value) = action.contractName.clone() {
            contract_manager_ref.get_contract_by_name(&String::from_utf8(value.to_vec())
                .map_err(|val|TransactionError{message:String::from("")})?)?
        } else {
            return Err(Box::new(TransactionError{message:String::from("")}));
        };

        println!("Before executing the action");
        let action_executor = self.action_manager.lock().unwrap().create_action_executor(&contract)?;
        action_executor.lock().unwrap().execute(transaction_state, &mut change_set)?;
        println!("After execution");

        let signed_change_set = SignedChangeSet{
            changeSet: change_set,
            changeSetHash: rasn::types::OctetString::new(),
            signature: rasn::types::OctetString::new(),
        };
        changeSets.push(signed_change_set);
        Ok(true)
    }

    fn incrementTransaction(&self, asnTransactionMessage: &mut asn1_lib::TransactionMessage) -> Result<(), Box<dyn std::error::Error>> {
        // we need to move the transction onto the next leg and create a new transaction message
        if asnTransactionMessage.transaction.currentStatus != Status::debit {
            return Ok(());
        }
        asnTransactionMessage.transaction.currentStatus = Status::credit;
        let bin_transaction = rasn::der::encode::<asn1_lib::TransactionMessage>(
            &asnTransactionMessage).unwrap();
        
        let transction_message = WebSocketMessage::Transaction(TransactionMessage::new(
            parse_octet_string(&asnTransactionMessage.transaction.sourceAccount)?, 
            parse_octet_string(&asnTransactionMessage.transaction.targetAccount)?, 
            peer2peer_protocol::CREDIT_TYPE.to_string().clone(), bin_transaction));
        
        self.transaction_processor_client_manager.create()?.
            submit_transaction(&serialize_bin_message(&transction_message)?)?;

        Ok(())
    }
}


#[async_trait]
impl TransactionProcessor for BlockTransactionProcessor {
    async fn process(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("From process1");
        let message = deserialize_bin_message(&self.transaction);
        println!("From process2");
        if message.is_err() {
            return Err(message.err().unwrap());
        }
        println!("From processing");
        if let WebSocketMessage::Transaction(trans) = message.unwrap() {
            println!("More processing");
            self.processTransactionMessage(&trans);
        }
        Ok(())
    }
}


pub trait TransactionProcessorFactory : Sync + Send {
    fn createProcessor(&self, transaction: &Vec<u8>) -> Arc<dyn TransactionProcessor>;
}

pub struct BlockTransactionProcessorFactory{
    session_factory: Arc<Mutex<dyn StoreSessionFactory>>,
    tangle_manager: Arc<Mutex<dyn TangleManager>>,
    contract_manager: Arc<Mutex<dyn ContractManager>>,
    action_manager: Arc<Mutex<dyn ActionManager>>,
    producer_client_manager: Arc<Mutex<dyn ProducerClientManager>>,
    transaction_processor_client_manager: Arc<dyn TransactionProcessorClientManager>,
}

impl TransactionProcessorFactory for BlockTransactionProcessorFactory {
    fn createProcessor(&self, transaction: &Vec<u8>) -> Arc<dyn TransactionProcessor> {
        BlockTransactionProcessor::new(transaction, &self.session_factory, &self.tangle_manager, &self.contract_manager, &self.action_manager, &self.producer_client_manager,
            &self.transaction_processor_client_manager)
    }
}

impl BlockTransactionProcessorFactory {
    pub fn new(session_factory: &Arc<Mutex<dyn StoreSessionFactory>>, tangle_manager: &Arc<Mutex<dyn TangleManager>>, 
        contract_manager: &Arc<Mutex<dyn ContractManager>>, action_manager: &Arc<Mutex<dyn ActionManager>>,
        producer_client_manager: &Arc<Mutex<dyn ProducerClientManager>>,
        transaction_processor_client_manager: &Arc<dyn TransactionProcessorClientManager>) -> Arc<dyn TransactionProcessorFactory> {
        let config = ChainConfig::new().unwrap();
        Arc::new(BlockTransactionProcessorFactory { 
            session_factory: session_factory.clone(), tangle_manager: tangle_manager.clone(), 
            contract_manager: contract_manager.clone(), action_manager: action_manager.clone(),
            producer_client_manager: producer_client_manager.clone(),
            transaction_processor_client_manager: transaction_processor_client_manager.clone() }) as Arc<dyn TransactionProcessorFactory>
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
    use crate::{Contract, MockActionManager, MockProducerClientManager};
    use chrono::prelude::*;
    use block_producer_client::mock::{MockTransactionProcessorClientManager};


    pub struct TransactionMockContract;

    impl TransactionMockContract {
        pub fn new() -> Result<Arc<Mutex<dyn Contract>>,Box<dyn std::error::Error>> {
            Ok(Arc::new(Mutex::new(TransactionMockContract{})))
        }
    }

    impl Contract for TransactionMockContract {
        fn contract_id(&self) -> Result<Vec<u8>,Box<dyn std::error::Error>> {
            Ok(String::from("TEST").into_bytes())
        }
        fn contract(&self) -> Result<Vec<u8>,Box<dyn std::error::Error>> {
            Ok(String::from("TEST").into_bytes())
        }
    }

    pub struct TransactionMockContractManager;
    
    impl TransactionMockContractManager {
        pub fn new() -> Result<Arc<Mutex<dyn ContractManager>>,Box<dyn std::error::Error>> {
            Ok(Arc::new(Mutex::new(TransactionMockContractManager {})) as Arc<Mutex<dyn ContractManager>>)
        }
    }
    
    impl ContractManager for TransactionMockContractManager {
        fn get_contract(&self,contract_id: &Vec<u8>) -> Result<Arc<Mutex<dyn Contract>>,Box<dyn std::error::Error>> {
            Ok(TransactionMockContract::new()?)
        }
        fn get_contract_by_name(&self,contract_name: &String) -> Result<Arc<Mutex<dyn Contract>>,Box<dyn std::error::Error>> {
            Ok(TransactionMockContract::new()?)
        }
    }

    fn create_transaction() -> Vec<u8> {
        
        let date: DateTime<Utc> = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
        let integer = 42u8; // Using a simple integer for this example

        let encoded_integer = rasn::der::encode(&integer).expect("Failed to encode integer");
        let asn1_action = asn1_lib::Action{
            version:1,
            date: date.clone(),
            contract: None,
            contractName: Some(rasn::types::OctetString::from("test")),
            parent: rasn::types::OctetString::from("parent"),
            model: rasn::types::Any::new(encoded_integer),
        };

        // this logic is here to confirm that the any in the model is getting serialized correctly
        let encoded_action_result = rasn::der::encode::<asn1_lib::Action>(
            &asn1_action);
        if encoded_action_result.is_err() {
            let error = encoded_action_result.unwrap_err();
            println!("An error occurred while encoding the message : {}",error.to_string());
            return vec![];
        }
        let mut asn1_action_copy_result = rasn::der::decode::<asn1_lib::Action>(
                &encoded_action_result.unwrap());
        if asn1_action_copy_result.is_err() {
            let error = asn1_action_copy_result.unwrap_err();
            println!("An error occurred while encoding the message : {}",error.to_string());
            return vec![];
        }
        let mut asn1_action_copy = asn1_action_copy_result.unwrap();
        println!("Contract name [{}][{}]",asn1_action.parent.escape_ascii().to_string(),
            asn1_action_copy.parent.escape_ascii().to_string());

        let asn1_transaction = asn1_lib::Transaction{
            version:1,
            date: date,
            value:1000,
            parent: rasn::types::OctetString::from("parent"),
            encrypted: false,
            sourceAccount: rasn::types::OctetString::from("sourceAccount"),
            targetAccount: rasn::types::OctetString::from("targetAccount"),
            transactionSignator: rasn::types::OctetString::from("transactionSignator"),
            creatorId: rasn::types::OctetString::from("creatorId"),
            actions: rasn::types::SequenceOf::from(vec![asn1_action.clone()]), 
        };
        println!("The length of actions is {}",asn1_transaction.actions.len());
        let asn1_signed_transaction = asn1_lib::SignedTransaction {
            version: 1,
            transaction: asn1_transaction,
            transactionHash: rasn::types::OctetString::from("transactionHash"),
            signature: rasn::types::OctetString::from("signature"),
        };
        println!("The length of actions is {}",asn1_signed_transaction.transaction.actions.len());
        let asn1_transaction_wrapper = asn1_lib::TransactionWrapper{
            version: 1,
            sourceAccount: rasn::types::OctetString::from("source"), 
            targetAccount: rasn::types::OctetString::from("target"),
            parent: rasn::types::OctetString::from("parent"),
            feeAccount: rasn::types::OctetString::from("feeAccount"),
            transactionHash: rasn::types::OctetString::from("feeAccount"),
            signature: rasn::types::OctetString::from("signature"),
            signedTransaction: asn1_signed_transaction,
            transactionTrace: Vec::new(),
            currentStatus: asn1_lib::Status::init,
            changeSet: Vec::new()
        };
        println!("The length of actions is {}",asn1_transaction_wrapper.signedTransaction.transaction.actions.len());
        let asn1_transaction= asn1_lib::TransactionMessage{
            version: 1,
            transaction: asn1_transaction_wrapper,
            availableTime: 1,
            elapsedTime: 0,
            sideTransactions: Vec::new(),
            encryptedSideTransactions: Vec::new()};
        println!("The length of actions is {}",asn1_transaction.transaction.signedTransaction.transaction.actions.len());
        let encoded_transaction = rasn::der::encode::<asn1_lib::TransactionMessage>(
            &asn1_transaction).unwrap();
        let mut asnTransactionMessage = rasn::der::decode::<asn1_lib::TransactionMessage>(
                &encoded_transaction).unwrap();
        println!("The length of actions is {}",asnTransactionMessage.transaction.signedTransaction.transaction.actions.len());
        println!("The to string of creator {}",asnTransactionMessage.transaction.signedTransaction.transaction.creatorId.escape_ascii().to_string());
        return peer2peer_protocol::serialize_message(&WebSocketMessage::Transaction(peer2peer_protocol::TransactionMessage{
            source_account_id: String::from("source"),
            target_account_id: String::from("target"),
            transaction_type: String::from("credit"), 
            binary_transaction: encoded_transaction})).unwrap().into_bytes();

    }

    pub struct TransactionMockTangleManager;


    impl TangleManager for TransactionMockTangleManager {
        fn managed_transaction(&self, transaction: &TransactionMessage) -> Result<bool,Box<dyn std::error::Error>> {
            Ok(true)
        }
        fn create_tangle(&self,account_ids: &Vec<Vec<u8>>) -> Result<Arc<Mutex<dyn crate::Tangle>>,Box<dyn std::error::Error>> {
            Ok(crate::mock::MockTangle::new()?)
        }
        fn get_tangle(&self,tangle_id: &Vec<u8>) -> Result<Arc<Mutex<dyn crate::Tangle>>,Box<dyn std::error::Error>> {
            Ok(crate::mock::MockTangle::new()?)
        }
        fn get_active_tangle(&self) -> Result<Option<Arc<Mutex<dyn crate::Tangle>>>,Box<dyn std::error::Error>> {
            Ok(Option::None)
        }
        fn set_active_tangle(&mut self,tangle_id: &Vec<u8>) -> Result<Option<Arc<Mutex<dyn crate::Tangle>>>,Box<dyn std::error::Error>> {
            Ok(Option::None)
        }
    }

    impl TransactionMockTangleManager {
        pub fn new() -> Result<Arc<Mutex<dyn TangleManager>>,Box<dyn std::error::Error>> {
            Ok(Arc::new(Mutex::new(TransactionMockTangleManager { })) as Arc<Mutex<dyn TangleManager>>)
        }
    }


    #[tokio::test]
    async fn test_block_processor_new() -> Result<(), Box<dyn Error>> {
        let transaction = create_transaction();
        let transaction_processor = BlockTransactionProcessor::new(
            &transaction, &MockStoreSessionFactory::new()?, 
            &TransactionMockTangleManager::new()?, &TransactionMockContractManager::new()?,
            &MockActionManager::new()?, &MockProducerClientManager::new()?,
            &MockTransactionProcessorClientManager::new());
        let _ = transaction_processor.process().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_block_transaction_processor_factory_new() -> Result<(), Box<dyn Error>> {
        let transaction = create_transaction();
        let _ = BlockTransactionProcessorFactory::new(
            &MockStoreSessionFactory::new()?, 
            &TransactionMockTangleManager::new()?,
            &TransactionMockContractManager::new()?,
            &MockActionManager::new()?,
            &MockProducerClientManager::new()?,
            &MockTransactionProcessorClientManager::new())
            .createProcessor(&transaction).process().await?;
        Ok(())
    }
}