use async_trait::async_trait;
use std::sync::{Arc,Mutex};
use config_lib::ChainConfig;
use rdf_lib::StoreSessionFactory;
use rdf_lib::store_result_set::*;
use rdf_store_client::{StoreClientManager,StoreClient};
use crate::{Contract};
use spool_client::{SpoolConnectionManager};
use asn1_lib::{ChangeData};


pub trait ActionExecutor: Sync + Send {
    fn execute(&self, transaction_state: &String, change_set: &mut asn1_lib::ChangeSet) -> Result<(),Box<dyn std::error::Error>>;
}

pub struct BlockActionExecutor {
    spool_manager: Arc<dyn SpoolConnectionManager>,
    contract: Arc<Mutex<dyn Contract>>,
}

impl BlockActionExecutor {
    pub fn new(spool_manager: &Arc<dyn SpoolConnectionManager>, contract: &Arc<Mutex<dyn Contract>>) -> Result<Arc<Mutex<dyn ActionExecutor>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new().unwrap();
        Ok(Arc::new(Mutex::new(BlockActionExecutor { spool_manager: spool_manager.clone(), contract: contract.clone() })))
    }
}

impl ActionExecutor for BlockActionExecutor {
    fn execute(&self, transaction_state: &String, change_set: &mut asn1_lib::ChangeSet) -> Result<(),Box<dyn std::error::Error>> {

        Ok(())
    }
}



