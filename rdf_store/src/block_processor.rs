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
        let block = rasn::der::decode::<asn1_lib::SignedBlock>(&block_bin).unwrap();

        let signed_block_hash = generate_hash_string(&block_bin);

        let blockSubject = format!("{}/{}",rdf_lib::RDFBlock::ID.to_string(),signed_block_hash.clone());
        let trippels = vec![
            asn1_lib::RDFNT::new(&blockSubject, &rdf_lib::RDFBlock::ID.to_string(), &signed_block_hash),
            asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFBlock::MERKLE_TREE_ROOT.to_string(), &hex::encode(tree.root_hash())),
            asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFBlock::TANGLE.to_string(), &self.id),
            asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFBlock::TANGLE_ID.to_string(), &self.id),
            asn1_lib::RDFNT::new(&blockSubject, &&rdf_lib::RDFBlock::DATA_BLOB.to_string(), &hex::encode(block_bin.clone())),];

        let sessionFactory = self.block_db_manager.sessionFactory()?;
        let sessionFactory = sessionFactory.lock().unwrap();
        let session = sessionFactory.createSession()?;
        let session = session.lock().unwrap();
        
        session.persistAsnTripples(&trippels);

        session.commit();

        Ok(())
    }
}