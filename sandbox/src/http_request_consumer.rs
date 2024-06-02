use std::sync::{Arc,Mutex};
use config_lib::ChainConfig;
use spool_client::{SpoolConnection,SpoolConnectionManager};
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use crate::{SandboxError, WasmExecutor, WasmExecutorManager};
use sandbox_client::{SANDBOX_HTTP_REQUEST_TOPIC,SandboxMessage,deserialize_http_sandbox_bin_message};

#[async_trait]
pub trait HttpRequestConsumer : Sync + Send{
    async fn process(&self);
}


pub struct SandboxHttpRequestConsumer {
    spool_manager: Arc<dyn SpoolConnectionManager>,
    wasm_executor_manager: Arc<Mutex<dyn WasmExecutorManager>>,
}


impl SandboxHttpRequestConsumer {
    pub fn new(spool_manager: &Arc<dyn SpoolConnectionManager>, wasm_executor_manager: &Arc<Mutex<dyn WasmExecutorManager>>) 
        -> Result<Arc<Mutex<dyn HttpRequestConsumer>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new().unwrap();
        Ok(Arc::new(Mutex::new(SandboxHttpRequestConsumer { spool_manager: spool_manager.clone(), wasm_executor_manager: wasm_executor_manager.clone() })))
    }
    pub fn process(&self, entry: &Vec<u8>) -> Result<(),Box<dyn std::error::Error>> {
        let wasm_executor_manager_ref = self.wasm_executor_manager.lock().unwrap();
        let request = deserialize_http_sandbox_bin_message(entry)?;
        let executor = wasm_executor_manager_ref.create_wasm_executor(&request.contract,&String::from("HTTP"))?;
        let executor_ref = executor.lock().unwrap();
        executor_ref.execute_http()?;
        Ok(())
    }
}

#[async_trait]
impl HttpRequestConsumer for SandboxHttpRequestConsumer {
    async fn process(&self) {
        let connection = 
            self.spool_manager.create_connection(&String::from(SANDBOX_HTTP_REQUEST_TOPIC)).await.unwrap();
        while let result = connection.consume().await {
            if result.is_err() {
                break;
            }
            let requests = result.unwrap();
            requests.iter().for_each(|entry| self.process(&entry).unwrap() );
        }
    }
}