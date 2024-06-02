use std::sync::{Arc,Mutex};
use config_lib::ChainConfig;
use spool_client::{SpoolConnection,SpoolConnectionManager};
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use crate::{SandboxError, WasmExecutor, WasmExecutorManager};
use sandbox_client::{SANDBOX_REQUEST_TOPIC,SandboxMessage,deserialize_sandbox_bin_message};

#[async_trait]
pub trait RequestConsumer : Sync + Send{
    async fn process(&self);
}


pub struct SandboxRequestConsumer {
    spool_manager: Arc<dyn SpoolConnectionManager>,
    wasm_executor_manager: Arc<Mutex<dyn WasmExecutorManager>>,
}


impl SandboxRequestConsumer {
    pub fn new(spool_manager: &Arc<dyn SpoolConnectionManager>, wasm_executor_manager: &Arc<Mutex<dyn WasmExecutorManager>>) 
        -> Result<Arc<Mutex<dyn RequestConsumer>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new().unwrap();
        Ok(Arc::new(Mutex::new(SandboxRequestConsumer { spool_manager: spool_manager.clone(), wasm_executor_manager: wasm_executor_manager.clone() })))
    }
    pub fn process(&self, entry: &Vec<u8>) -> Result<(),Box<dyn std::error::Error>> {
        let wasm_executor_manager_ref = self.wasm_executor_manager.lock().unwrap();
        let request = deserialize_sandbox_bin_message(entry)?;
        let executor = wasm_executor_manager_ref.create_wasm_executor(&request.contract,&request.action_type)?;
        let executor_ref = executor.lock().unwrap();
        let mut asn1_action_copy_result = rasn::der::decode::<asn1_lib::ChangeSet>(
            &request.change_set.clone());
        if asn1_action_copy_result.is_err() {
            return Err(Box::new(SandboxError{message:String::from("Could not extract the change set")}));
        }
        executor_ref.execute(&mut asn1_action_copy_result.unwrap())?;
        Ok(())
    }
}

#[async_trait]
impl RequestConsumer for SandboxRequestConsumer {
    async fn process(&self) {
        let connection = 
            self.spool_manager.create_connection(&String::from(SANDBOX_REQUEST_TOPIC)).await.unwrap();
        while let result = connection.consume().await {
            if result.is_err() {
                break;
            }
            let requests = result.unwrap();
            requests.iter().for_each(|entry| self.process(&entry).unwrap() );
        }
    }
}