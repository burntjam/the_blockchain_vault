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
use super::account::*;
use rdf_lib::{StoreSessionFactory};
use std::time::Duration;


pub trait AccountManager: Sync + Send {
    fn create_account(&mut self,account_id: &Vec<u8>, tangle_id: &Vec<u8>) -> Result<Arc<Mutex<dyn Account>>,Box<dyn std::error::Error>>;
    fn get_account(&mut self,account_id: &Vec<u8>) -> Result<Option<Arc<Mutex<dyn Account>>>,Box<dyn std::error::Error>>;   
}

pub struct BlockAccountManager {
    session_factory: Arc<Mutex<dyn StoreSessionFactory>>,
    accounts: HashMap<Vec<u8>,Arc<Mutex<dyn Account>>>,
}

impl BlockAccountManager {
    pub fn new(session_factory: Arc<Mutex<dyn StoreSessionFactory>>) -> Result<Arc<Mutex<dyn AccountManager>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(Mutex::new(BlockAccountManager { session_factory, accounts: HashMap::new() })) as Arc<Mutex<dyn AccountManager>>)
    }
}

impl AccountManager for BlockAccountManager {
    fn create_account(&mut self,account_id: &Vec<u8>, tangle_id: &Vec<u8>) -> Result<Arc<Mutex<dyn Account>>,Box<dyn std::error::Error>> {
        let account = BlockAccount::new_account(account_id, tangle_id, self.session_factory.clone())?;
        self.accounts.insert(account_id.clone(), account.clone());
        Ok(account)
    }
    fn get_account(&mut self,account_id: &Vec<u8>) -> Result<Option<Arc<Mutex<dyn Account>>>,Box<dyn std::error::Error>> {
        let mut entry  = self.accounts.get(account_id).cloned();
        if entry.is_none() {
            let account = BlockAccount::new_instance(account_id, self.session_factory.clone())?;
            self.accounts.insert(account_id.clone(), account.clone());
            entry = self.accounts.get(account_id).cloned();
        } else if let Some(value) = entry.clone() {
            if value.lock().unwrap().expired()? {
                let account = BlockAccount::new_instance(account_id, self.session_factory.clone())?;
                self.accounts.insert(account_id.clone(), account.clone());
                entry = self.accounts.get(account_id).cloned();
            }
        }
        Ok(entry)
    }
}

