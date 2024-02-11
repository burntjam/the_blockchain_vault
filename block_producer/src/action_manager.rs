use oxigraph::store::Store;
use oxigraph::model::*;
use oxigraph::sparql::QueryResults;
use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use crate::{ActionExecutor,Contract,BlockActionExecutor};
use spool_client::{SpoolConnectionManager};

pub trait ActionManager: Sync + Send {
    fn create_action_executor(&self,contract: &Arc<Mutex<dyn Contract>>) -> Result<Arc<Mutex<dyn ActionExecutor>>,Box<dyn std::error::Error>>;
}

pub struct BlockActionManager {
    spool_manager: Arc<dyn SpoolConnectionManager>,
}

impl BlockActionManager {
    pub fn new(spool_manager: &Arc<dyn SpoolConnectionManager>) -> Result<Arc<Mutex<dyn ActionManager>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new().unwrap();
        Ok(Arc::new(Mutex::new(BlockActionManager { spool_manager: spool_manager.clone() })))
    }
}

impl ActionManager for BlockActionManager {
    fn create_action_executor(&self,contract: &Arc<Mutex<dyn Contract>>) -> Result<Arc<Mutex<dyn ActionExecutor>>,Box<dyn std::error::Error>> {
        Ok(BlockActionExecutor::new(&self.spool_manager, contract)?)
    }
}