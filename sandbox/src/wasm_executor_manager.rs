use oxigraph::store::Store;
use oxigraph::model::*;
use oxigraph::sparql::QueryResults;
use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use crate::{wasm_handler_factory, SandboxWasmExecutor, WasmExecutor, WasmHandlerFactory};
use spool_client::{SpoolConnectionManager};

pub trait WasmExecutorManager: Sync + Send {
    fn create_wasm_executor(&self, contract: &Vec<u8>, action_type: &String) -> Result<Arc<Mutex<dyn WasmExecutor>>,Box<dyn std::error::Error>>;
}

pub struct SandboxWasmExecutorManager {
    spool_manager: Arc<dyn SpoolConnectionManager>,
    wasm_handler_factory: Arc<Mutex<dyn WasmHandlerFactory>>,
}

impl SandboxWasmExecutorManager {
    pub fn new(spool_manager: &Arc<dyn SpoolConnectionManager>, wasm_handler_factory: &Arc<Mutex<dyn WasmHandlerFactory>>) -> Result<Arc<Mutex<dyn WasmExecutorManager>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new().unwrap();
        Ok(Arc::new(Mutex::new(SandboxWasmExecutorManager { spool_manager: spool_manager.clone(), wasm_handler_factory: wasm_handler_factory.clone() })))
    }
}

impl WasmExecutorManager for SandboxWasmExecutorManager {
    fn create_wasm_executor(&self, contract: &Vec<u8>, action_type: &String) -> Result<Arc<Mutex<dyn WasmExecutor>>,Box<dyn std::error::Error>> {
        Ok(SandboxWasmExecutor::new(&self.spool_manager, contract, action_type, &self.wasm_handler_factory)?)
    }
}