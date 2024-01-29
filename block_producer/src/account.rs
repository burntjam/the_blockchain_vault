use std::sync::{Arc,Mutex};
use config_lib::ChainConfig;
use spool_client::{SpoolConnection};
use super::mock::{MockTransactionManager};
use super::transaction_constants::*;
use super::transaction_manager::*;
use rdf_lib::{StoreSessionFactory};
use chrono::{DateTime, Utc};


pub trait Account: Sync + Send {
    fn account_id(&self) -> Result<Vec<u8>,Box<dyn std::error::Error>>;
    fn tangle_id(&self) -> Result<Vec<u8>,Box<dyn std::error::Error>>;
    fn expired(&self) -> Result<bool,Box<dyn std::error::Error>>;
}

pub struct BlockAccount {
    account_id: Vec<u8>,
    tangle_id: Vec<u8>,
    session_factory: Arc<Mutex<dyn StoreSessionFactory>>,
    touch_time: DateTime<Utc>,
}

impl BlockAccount {
    pub fn new_account(account_id: &Vec<u8>, tangle_id: &Vec<u8>, session_factory: Arc<Mutex<dyn StoreSessionFactory>>) -> Result<Arc<Mutex<dyn Account>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        {
            let session_factory_ref = session_factory.lock().unwrap();
            let session = session_factory_ref.createSession()?;
            let session_ref = session.lock().unwrap();
            let entries = session_ref.query(&"SELECT ".to_string())?;
        }
        Ok(Arc::new(Mutex::new(BlockAccount { account_id: account_id.clone(), tangle_id: tangle_id.clone(), session_factory, touch_time: Utc::now() })) as Arc<Mutex<dyn Account>>)
    }

    pub fn new_instance(account_id: &Vec<u8>, session_factory: Arc<Mutex<dyn StoreSessionFactory>>) -> Result<Arc<Mutex<dyn Account>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(Mutex::new(BlockAccount { account_id: account_id.clone(), tangle_id: account_id.clone(), session_factory, touch_time: Utc::now() })) as Arc<Mutex<dyn Account>>)
    }
}


impl Account for BlockAccount {
    fn account_id(&self) -> Result<Vec<u8>,Box<dyn std::error::Error>> {
        Ok(self.account_id.clone())
    }
    fn tangle_id(&self) -> Result<Vec<u8>,Box<dyn std::error::Error>> {
        Ok(self.tangle_id.clone())
    }
    fn expired(&self) -> Result<bool,Box<dyn std::error::Error>> {
        let difference = Utc::now() - self.touch_time;
        Ok(difference.num_seconds() > (10 * 60))
    }
}

