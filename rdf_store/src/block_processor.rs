use async_trait::async_trait;
use rdf_lib::RdfResultSet;
use spool_client::*;
use config_lib::ChainConfig;
use std::sync::{Arc, Mutex};
use rdf_store_client::message::*;
use crate::DiskStoreManager;
use sha2::{Sha256, Digest};


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
pub trait BlockProcessor: Sync + Send {
    async fn process(&self, block_bin: &Vec<u8>) -> Result<(),Box<dyn std::error::Error>>;
}

pub struct DiskBlockProcessor {
    disk_store_manager: Arc<dyn DiskStoreManager>,
    spool_manager: Arc<dyn SpoolConnectionManager>, 
}

impl DiskBlockProcessor {
    pub fn new(disk_store_manager: &Arc<dyn DiskStoreManager>, spool_manager: &Arc<dyn SpoolConnectionManager>) -> Result<Arc<dyn BlockProcessor>, Box<dyn std::error::Error>> {
        let config = ChainConfig::new().unwrap();
        Ok(Arc::new(DiskBlockProcessor { disk_store_manager: disk_store_manager.clone(), spool_manager: spool_manager.clone() }))
    }
}


#[async_trait]
impl BlockProcessor for DiskBlockProcessor {
    async fn process(&self, block_bin: &Vec<u8>) -> Result<(),Box<dyn std::error::Error>> {
        let signed_block = rasn::der::decode::<asn1_lib::SignedBlock>(&block_bin).unwrap();

        
        let blockSubject = format!("{}/{}",rdf_lib::RDFBlock::ID.to_string(),&hex::encode(&signed_block.hash));
        let mut trippels = vec![
            asn1_lib::RDFNT::new(&blockSubject, &rdf_lib::RDFBlock::ID.to_string(), &hex::encode(&signed_block.hash)),
            asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFBlock::MERKLE_TREE_ROOT.to_string(), &hex::encode(&signed_block.block.merkelRoot)),
            asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFBlock::TANGLE.to_string(), &hex::encode(&signed_block.tangle_hash)),
            asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFBlock::TANGLE_ID.to_string(), &hex::encode(&signed_block.tangle_hash)),
            asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFBlock::DATA_BLOB.to_string(), &hex::encode(&block_bin)),];

        signed_block.block.transactions.iter().for_each(|transaction| {
            let transaction_blob = rasn::der::encode::<asn1_lib::TransactionWrapper>(transaction).unwrap();
            let transaction_tripples = vec![
                asn1_lib::RDFNT::new(&blockSubject, &rdf_lib::RDFTransaction::ID.to_string(), &hex::encode(&transaction.signedTransaction.transactionHash)),
                asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFTransaction::DEBIT_ACCOUNT.to_string(), &hex::encode(&transaction.sourceAccount)),
                asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFTransaction::DEBIT_ACCOUNT_ID.to_string(), &hex::encode(&transaction.sourceAccount)),
                asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFTransaction::CREDIT_ACCOUNT.to_string(), &hex::encode(&transaction.targetAccount)),
                asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFTransaction::CREDIT_ACCOUNT_ID.to_string(), &hex::encode(&transaction.targetAccount)),
                asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFTransaction::VALUE.to_string(), &format!("{}",&transaction.signedTransaction.transaction.value)),
                asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFTransaction::VALUE.to_string(), &format!("{}",&transaction.signedTransaction.transaction.value)),

                asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFBlock::TANGLE.to_string(), &hex::encode(&signed_block.tangle_hash)),
                asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFBlock::TANGLE_ID.to_string(), &hex::encode(&signed_block.tangle_hash)),
                asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFBlock::DATA_BLOB.to_string(), &hex::encode(&block_bin)),];
        });

        
        let sessionFactory = self.disk_store_manager.get_session_factory()?;
        let sessionFactory = sessionFactory.lock().unwrap();
        let session = sessionFactory.createSession()?;
        let session = session.lock().unwrap();
        
        session.persistAsnTripples(&trippels);

        session.commit();

        Ok(())
    }
}