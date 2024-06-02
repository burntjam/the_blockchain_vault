use std::sync::{Arc,Mutex};
use config_lib::ChainConfig;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use crate::wasm_handler::{WasmHandler,SandboxWasmHandler};


pub trait WasmHandlerFactory: Sync + Send {
    fn create_handler(&self) -> Result<Arc<Mutex<dyn WasmHandler>>,Box<dyn std::error::Error>>;
}

pub struct SandboxWasmHandlerFactory {
}


impl SandboxWasmHandlerFactory {
    pub fn new() -> Result<Arc<Mutex<dyn WasmHandlerFactory>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new().unwrap();
        Ok(Arc::new(Mutex::new(SandboxWasmHandlerFactory { })))
    }
}


impl WasmHandlerFactory for SandboxWasmHandlerFactory {
    fn create_handler(&self) -> Result<Arc<Mutex<dyn WasmHandler>>,Box<dyn std::error::Error>> {
        Ok(SandboxWasmHandler::new()?)
    }
}
