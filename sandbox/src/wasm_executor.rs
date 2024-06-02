use async_trait::async_trait;
use std::sync::{Arc,Mutex};
use config_lib::ChainConfig;
use rdf_lib::StoreSessionFactory;
use rdf_lib::store_result_set::*;
use rdf_store_client::{StoreClientManager,StoreClient};
use spool_client::{SpoolConnectionManager};
use asn1_lib::{ChangeData};
use crate::SandboxError;
use crate::{WasmHandlerFactory,WasmHandler};




pub trait WasmExecutor: Sync + Send {
    fn execute(&self,change_set: &mut asn1_lib::ChangeSet) -> Result<(),Box<dyn std::error::Error>>;
    fn execute_http(&self) -> Result<(),Box<dyn std::error::Error>>;
}

pub struct SandboxWasmExecutor {
    spool_manager: Arc<dyn SpoolConnectionManager>,
    contract: Vec<u8>,
    action_type: String,
    wasm_handler_factory: Arc<Mutex<dyn WasmHandlerFactory>>,
}


impl SandboxWasmExecutor {
    pub fn new(spool_manager: &Arc<dyn SpoolConnectionManager>, contract: &Vec<u8>,action_type: &String, wasm_handler_factory: &Arc<Mutex<dyn WasmHandlerFactory>>) 
        -> Result<Arc<Mutex<dyn WasmExecutor>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new().unwrap();
        Ok(Arc::new(Mutex::new(SandboxWasmExecutor { spool_manager: spool_manager.clone(), contract: contract.clone(), 
            action_type: action_type.clone(), wasm_handler_factory:  wasm_handler_factory.clone()})))
    }
    async fn async_execute(&self, handler: &Arc<Mutex<dyn WasmHandler>>, change_set: &mut asn1_lib::ChangeSet) -> Result<(),Box<dyn std::error::Error>> {
        let mut handler_ref = handler.lock().unwrap();
        if self.action_type.eq_ignore_ascii_case(&"debit") {
            let _ = handler_ref.execute_bytes_with_changeset(&self.contract, &"run_debit",change_set).await?;
        } else if self.action_type.eq_ignore_ascii_case(&"credit") {
            let _ = handler_ref.execute_bytes_with_changeset(&self.contract, &"run_credit",change_set).await?;
        } else if self.action_type.eq_ignore_ascii_case(&"http") {
            let _ = handler_ref.execute_bytes(&self.contract, &"run_http").await?;
        } else {
            return Err(Box::new(SandboxError{message:format!("Invalid action type {}",self.action_type)}));
        }
        
        Ok(())
    }

    async fn async_execute_http(&self, handler: &Arc<Mutex<dyn WasmHandler>>) -> Result<(),Box<dyn std::error::Error>> {
        let mut handler_ref = handler.lock().unwrap();
        let _ = handler_ref.execute_bytes(&self.contract, &"run_http").await?;
        Ok(())
    }
}

impl WasmExecutor for SandboxWasmExecutor {
    fn execute(&self,change_set: &mut asn1_lib::ChangeSet) -> Result<(),Box<dyn std::error::Error>> {
        let wasm_handler_factory_ref = self.wasm_handler_factory.lock().unwrap();
        let handler: Arc<Mutex<dyn WasmHandler>> = wasm_handler_factory_ref.create_handler()?;
        let _ = futures::executor::block_on(self.async_execute(&handler,change_set))?;
        Ok(())
    }
    fn execute_http(&self) -> Result<(),Box<dyn std::error::Error>> {
        let wasm_handler_factory_ref = self.wasm_handler_factory.lock().unwrap();
        let handler: Arc<Mutex<dyn WasmHandler>> = wasm_handler_factory_ref.create_handler()?;
        let _ = futures::executor::block_on(self.async_execute_http(&handler))?;
        Ok(())
    }
}

