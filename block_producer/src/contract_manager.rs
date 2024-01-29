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
use super::contract::*;
use rdf_lib::{StoreSessionFactory};


pub trait ContractManager: Sync + Send {
    fn get_contract(&self,contract_id: &Vec<u8>) -> Result<Arc<Mutex<dyn Contract>>,Box<dyn std::error::Error>>;
}

pub struct BlockContractManager {
    session_factory: Arc<Mutex<dyn StoreSessionFactory>>,
}

impl BlockContractManager {
    pub fn new(session_factory: Arc<Mutex<dyn StoreSessionFactory>>) -> Result<Arc<Mutex<dyn ContractManager>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(Mutex::new(BlockContractManager { session_factory })) as Arc<Mutex<dyn ContractManager>>)
    }
}

impl ContractManager for BlockContractManager {
    fn get_contract(&self,contract_id: &Vec<u8>) -> Result<Arc<Mutex<dyn Contract>>,Box<dyn std::error::Error>> {
        Ok(BlockContract::new(contract_id, &vec![1,2], self.session_factory.clone())?)
    }
}