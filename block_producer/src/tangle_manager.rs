use std::collections::HashMap;
use std::sync::{Arc,Mutex};
use config_lib::ChainConfig;
use spool_client::{SpoolConnection};
use super::mock::{MockTransactionManager};
use super::transaction_constants::*;
use super::transaction_manager::*;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use peer2peer_protocol::TransactionMessage;
use super::tangle::*;
use rdf_lib::{StoreSessionFactory};

pub trait TangleManager: Sync + Send {
    fn managed_transaction(&self, transaction: &TransactionMessage) -> Result<bool,Box<dyn std::error::Error>>;
    fn create_tangle(&self,account_ids: &Vec<Vec<u8>>) -> Result<Arc<Mutex<dyn Tangle>>,Box<dyn std::error::Error>>;
    fn get_tangle(&self,tangle_id: &Vec<u8>) -> Result<Arc<Mutex<dyn Tangle>>,Box<dyn std::error::Error>>;
    fn get_active_tangle(&self) -> Result<Option<Arc<Mutex<dyn Tangle>>>,Box<dyn std::error::Error>>;
    fn set_active_tangle(&mut self,tangle_id: &Vec<u8>) -> Result<Option<Arc<Mutex<dyn Tangle>>>,Box<dyn std::error::Error>>;
}

pub struct BlockTangleManager {
    active_tangle: Option<Arc<Mutex<dyn Tangle>>>,
    tangles: HashMap<String,Arc<Mutex<dyn Tangle>>>,
    session_factory: Arc<Mutex<dyn StoreSessionFactory>>,
}

impl BlockTangleManager {
    pub fn new(tangle_id: Vec<u8>, session_factory: Arc<Mutex<dyn StoreSessionFactory>>) -> Result<Arc<Mutex<dyn TangleManager>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(Mutex::new(BlockTangleManager { active_tangle: Option::None, tangles: HashMap::new(), session_factory })) as Arc<Mutex<dyn TangleManager>>)
    }
    pub fn init(session_factory: Arc<Mutex<dyn StoreSessionFactory>>) -> Result<Arc<Mutex<dyn TangleManager>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(Mutex::new(BlockTangleManager { active_tangle: Option::None, tangles: HashMap::new(), session_factory })) as Arc<Mutex<dyn TangleManager>>)
    }
}

impl TangleManager for BlockTangleManager {
    fn managed_transaction(&self, transaction: &TransactionMessage) -> Result<bool,Box<dyn std::error::Error>> {
        if let Some(active_tangle) = self.active_tangle.clone() {
            let active_tangle_ref = active_tangle.lock().unwrap();
            return Ok(active_tangle_ref.tangle_accounts().unwrap().iter().any(|account| 
                String::from_utf8(account.clone()).unwrap() == transaction.getAccountId().clone()));
        }
        Ok(false)
    }

    fn create_tangle(&self,account_ids: &Vec<Vec<u8>>) -> Result<Arc<Mutex<dyn Tangle>>,Box<dyn std::error::Error>> {
        let tangle = BlockTangle::new_tangle(account_ids, self.session_factory.clone())?;
        Ok(tangle)
    }

    fn get_tangle(&self,tangle_id: &Vec<u8>) -> Result<Arc<Mutex<dyn Tangle>>,Box<dyn std::error::Error>> {
        let tangle = BlockTangle::new(tangle_id.clone(), self.session_factory.clone())?;
        Ok(tangle)
    }

    fn get_active_tangle(&self) -> Result<Option<Arc<Mutex<dyn Tangle>>>,Box<dyn std::error::Error>> {
        Ok(self.active_tangle.clone())
    }

    fn set_active_tangle(&mut self,tangle_id: &Vec<u8>) -> Result<Option<Arc<Mutex<dyn Tangle>>>,Box<dyn std::error::Error>> {
        self.active_tangle = Some(self.get_tangle(tangle_id)?);
        Ok(self.active_tangle.clone())
    }
}