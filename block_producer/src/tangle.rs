use std::sync::{Arc,Mutex};
use config_lib::ChainConfig;
use rdf_lib::StoreSessionFactory;

pub trait Tangle: Sync + Send {
    fn tangle_id(&self) -> Result<Vec<u8>,Box<dyn std::error::Error>>;
    fn tangle_accounts(&self) -> Result<Vec<Vec<u8>>,Box<dyn std::error::Error>>;
    fn add_account(&self,account_id: Vec<u8>) -> Result<(),Box<dyn std::error::Error>>;
    fn verify_account(&self,account_id: Vec<u8>) -> Result<bool,Box<dyn std::error::Error>>;
}

pub struct BlockTangle {
    tangle_id: Vec<u8>,
    session_factory: Arc<Mutex<dyn StoreSessionFactory>>,
}

impl BlockTangle {
    pub fn new(tangle_id: Vec<u8>, session_factory: Arc<Mutex<dyn StoreSessionFactory>>) -> Result<Arc<Mutex<dyn Tangle>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(Mutex::new(BlockTangle { tangle_id, session_factory })) as Arc<Mutex<dyn Tangle>>)
    }
    pub fn new_tangle(account_ids: &Vec<Vec<u8>>, session_factory: Arc<Mutex<dyn StoreSessionFactory>>) -> Result<Arc<Mutex<dyn Tangle>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(Mutex::new(BlockTangle { tangle_id:Vec::new(), session_factory })) as Arc<Mutex<dyn Tangle>>)
    }
}

impl Tangle for BlockTangle {
    fn tangle_id(&self) -> Result<Vec<u8>,Box<dyn std::error::Error>> {
        Ok(self.tangle_id.clone())
    }
    fn tangle_accounts(&self) -> Result<Vec<Vec<u8>>,Box<dyn std::error::Error>> {
        Ok(Vec::new())
    }   
    fn add_account(&self,account_id: Vec<u8>) -> Result<(),Box<dyn std::error::Error>> {
        Ok(())
    }
    fn verify_account(&self,account_id: Vec<u8>) -> Result<bool,Box<dyn std::error::Error>> {
        Ok(false)
    }
}