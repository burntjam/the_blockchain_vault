use std::io::Read;
use std::sync::{Arc,Mutex};
use chrono::DateTime;
use config_lib::ChainConfig;
use peer2peer_protocol::server::transaction;
use peer2peer_protocol::{deserialize_bin_message, WebSocketMessage, handler::message, TransactionMessage};
use rdf_lib::{StoreSessionFactory};
use async_trait::async_trait;
use ring::hmac::sign;
use std::future::Future;
use std::pin::Pin;
use chrono::prelude::*;
use crate::tangle_manager::TangleManager;
use crate::{action_executor, producer_client_manager, ActionExecutor, ActionManager, Contract, ContractManager, ProducerClientManager, TransactionError, DbManager};
use asn1_lib::{ChangeSet, SignedChangeSet, SignedTransaction, SoftwareConsensus, Transaction, TransactionWrapper};
use sha2::{Sha256, Digest};
use generic_array::GenericArray;
use generic_array::typenum::U32;
use merkle::MerkleTree;
use ring::digest::{Algorithm, SHA256}; // Import SHA256 or whichever algorithm you're using.
use rdf_store_client::{store_client_manager, StoreClientManager};


#[async_trait]
pub trait TangleProducer: Sync + Send {
    async fn process(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn id(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
    async fn is_valid_transaction(&self,transaction: &TransactionMessage) -> bool;
    async fn submit_transaction(&self,transaction: &TransactionMessage) -> Result<(), Box<dyn std::error::Error>>;
}


pub struct BlockTangleProducer {
    id: Vec<u8>,
    current_block: Arc<Mutex<Vec<u8>>>,
    transactions: Arc<Mutex<Vec<TransactionMessage>>>,
    block_db_manager: Arc<dyn DbManager>,
    store_client_manager: Arc<Mutex<dyn StoreClientManager>>,
}


impl BlockTangleProducer {
    pub async fn new(id: &Vec<u8>,block_db_manager: &Arc<dyn DbManager>, store_client_manager: &Arc<Mutex<dyn StoreClientManager>>) -> Result<Arc<dyn TangleProducer>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new().unwrap();

        let sessionFactory = block_db_manager.sessionFactory()?;
        let sessionFactory = sessionFactory.lock().unwrap();
        let session = sessionFactory.createSession()?;
        let session = session.lock().unwrap();
        


        Ok(Arc::new(BlockTangleProducer {id: id.clone(), current_block: Arc::new(Mutex::new(id.clone())) , transactions: Arc::new(Mutex::new(Vec::new())), 
            block_db_manager: block_db_manager.clone(), store_client_manager: store_client_manager.clone()}) as Arc<dyn TangleProducer>)
    }
}

fn generate_hash_string(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    format!("{:x}", result) // Converts hash to hex string
}

fn generate_hash(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}


#[async_trait]
impl TangleProducer for BlockTangleProducer {
    async fn process(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut transactions = self.transactions.lock().unwrap();
        
        let mut block = asn1_lib::Block::blank_block();
        let mut hashes: Vec<Vec<u8>> = Vec::new();
        

        transactions.iter().for_each(|transactionMessage| {
            let mut asnTransactionMessage = rasn::der::decode::<asn1_lib::TransactionMessage>(
                &transactionMessage.binary_transaction.clone()).unwrap();
            block.transactions.push(asnTransactionMessage.transaction.clone());
            let signed_transaction_bin = rasn::der::encode::<asn1_lib::Transaction>(
                &asnTransactionMessage.transaction.signedTransaction.transaction).unwrap();
            hashes.push(generate_hash(&signed_transaction_bin));
        });
        
        let tree = MerkleTree::from_vec(&SHA256, hashes);
        block.merkelRoot = rasn::types::OctetString::from(tree.root_hash().clone());
        let mut current_block = self.current_block.lock().unwrap();
        block.parent = rasn::types::OctetString::from(current_block.clone());

        let block_bin = rasn::der::encode::<asn1_lib::Block>(&block).unwrap();
        let block_hash = generate_hash(&block_bin);
        let block_hash_hex = hex::encode(&block_hash);

        let mut signed_block = asn1_lib::SignedBlock::new(&block);

        
        
        
        let signed_block_bin = rasn::der::encode::<asn1_lib::SignedBlock>(&signed_block).unwrap();
        
        
        let blockSubject = format!("{}/{}",rdf_lib::RDFBlock::ID.to_string(),&block_hash_hex);
        let trippels = vec![
            asn1_lib::RDFNT::new(&blockSubject, &rdf_lib::RDFBlock::ID.to_string(), &block_hash_hex),
            asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFBlock::MERKLE_TREE_ROOT.to_string(), &hex::encode(tree.root_hash())),
            asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFBlock::TANGLE.to_string(), &hex::encode(&self.id)),
            asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFBlock::TANGLE_ID.to_string(), &hex::encode(&self.id)),
            asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFBlock::DATA_BLOB.to_string(), &hex::encode(signed_block_bin.clone())),];

        let sessionFactory = self.block_db_manager.sessionFactory()?;
        let sessionFactory = sessionFactory.lock().unwrap();
        let session = sessionFactory.createSession()?;
        let session = session.lock().unwrap();
        
        session.persistAsnTripples(&trippels);

        session.commit();

        let store_client_manager = self.store_client_manager.lock().unwrap();
        let store_client = store_client_manager.create_client()?;
        let store_client = store_client.lock().unwrap();
        store_client.persist_signed_block(&signed_block_bin);

        current_block.clear();
        current_block.extend(block_hash);

        Ok(())
    }
    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    async fn id(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(self.id.clone())
    }
    async fn is_valid_transaction(&self,transaction: &TransactionMessage) -> bool {
        transaction.getAccountId();
        true
    }
    async fn submit_transaction(&self,transaction: &TransactionMessage) -> Result<(), Box<dyn std::error::Error>> {
        let mut transactions = self.transactions.lock().unwrap();
        transactions.push(transaction.clone());
        Ok(())
    }
}


