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
use rdf_store_client::StoreClientManager;


pub trait ContractManager: Sync + Send {
    fn get_contract(&self,contract_id: &Vec<u8>) -> Result<Arc<Mutex<dyn Contract>>,Box<dyn std::error::Error>>;
    fn get_contract_by_name(&self,contract_name: &String) -> Result<Arc<Mutex<dyn Contract>>,Box<dyn std::error::Error>>;
}

pub struct BlockContractManager {
    store_client_manager: Arc<Mutex<dyn StoreClientManager>>,
}

impl BlockContractManager {
    pub fn new(store_client_manager: &Arc<Mutex<dyn StoreClientManager>>) -> Result<Arc<Mutex<dyn ContractManager>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(Mutex::new(BlockContractManager { store_client_manager: store_client_manager.clone()  })) as Arc<Mutex<dyn ContractManager>>)
    }
}

impl ContractManager for BlockContractManager {
    fn get_contract(&self,contract_id: &Vec<u8>) -> Result<Arc<Mutex<dyn Contract>>,Box<dyn std::error::Error>> {
        Ok(BlockContract::new(contract_id, &String::from(""),  &self.store_client_manager)?)
    }
    fn get_contract_by_name(&self,contract_name: &String) -> Result<Arc<Mutex<dyn Contract>>,Box<dyn std::error::Error>> {
        Ok(BlockContract::new(&vec![], contract_name,  &self.store_client_manager)?)
    }
}