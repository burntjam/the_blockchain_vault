use async_trait::async_trait;
use std::sync::{Arc,Mutex};
use wasmtime::{Caller, Extern, Func, FuncType, ImportType, Instance, Linker, Store, Trap, Val, ValType};


#[async_trait]
pub trait WasmHandler {
    async fn execute_url(&mut self, url: &str, function: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn execute_bytes(&mut self, bytes: &Vec<u8>, function: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn execute_bytes_with_changeset(&mut self, bytes: &Vec<u8>, function: &str, change_set: &mut asn1_lib::ChangeSet) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct SandboxWasmHandler {
    pub store: Store<()>,
    pub instance: Option<Instance>,
    //callback: Arc<dyn Fn(i32) + Send + Sync>,
}


impl SandboxWasmHandler {
    pub fn new() -> Result<Arc<Mutex<dyn WasmHandler>>,Box<dyn std::error::Error>> {
        Ok(Arc::new(Mutex::new(SandboxWasmHandler {
            store: Store::default(),
            instance: None,
            //callback: Arc::new(callback),
        })))
    }

    pub fn receive_message(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let receive_message = self.get_func("receive_message").unwrap();
        receive_message.call(&mut self.store,&[], &mut [])?;
        Ok(())
    }

    pub fn get_func(&mut self, name: &str) -> Result<Func, Box<dyn std::error::Error>> {
        Ok(self.instance.as_ref().ok_or("No WASM instance loaded")?.get_func(&mut self.store, name).ok_or("Function not found")?)
    }


    fn action_callbacks(&mut self) -> Result<Box<Linker<()>>, Box<dyn std::error::Error>> {
        let mut linker = Box::new(Linker::new(self.store.engine()));
        let callback = {
            move |ptr: i32|  {
                
            }
        };

        let callback_func = Func::wrap(&mut self.store, callback);

        linker.define( &self.store, "cdv","callback",callback_func);

        Ok(linker)
    }

    fn http_callbacks(&mut self) -> Result<Box<Linker<()>>, Box<dyn std::error::Error>> {
        let mut linker = Box::new(Linker::new(self.store.engine()));
        let callback = {
            move |ptr: i32|  {
                
            }
        };

        let callback_func = Func::wrap(&mut self.store, callback);

        linker.define( &self.store, "cdv","callback",callback_func);

        Ok(linker)
    }

}


#[async_trait]
impl WasmHandler for SandboxWasmHandler {
    async fn execute_url(&mut self, url: &str, function: &str) -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::get(url).await?;
        let wasm_bytes = response.bytes().await?;
        let module = wasmtime::Module::new(&self.store.engine(), wasm_bytes.as_ref())?;
        let instance = wasmtime::Instance::new(&mut self.store, &module, &[])?;
        self.instance = Some(instance);

        // Initialize the module
        let start = self.get_func(function)?;
        start.call(&mut self.store,&[], &mut [])?;
        Ok(())
    }
    async fn execute_bytes(&mut self, bytes: &Vec<u8>, function: &str) -> Result<(), Box<dyn std::error::Error>> {
        let linker = self.http_callbacks()?;
        let module = wasmtime::Module::new(&self.store.engine(), &bytes)?;
        let instance = linker.instantiate(&mut self.store,&module)?;
        
        self.instance = Some(instance);

        // Initialize the module
        let start = self.get_func(function)?;
        start.call(&mut self.store,&[], &mut [])?;
        Ok(())
    }
    async fn execute_bytes_with_changeset(&mut self, bytes: &Vec<u8>, function: &str, change_set: &mut asn1_lib::ChangeSet) -> Result<(), Box<dyn std::error::Error>> {
        let linker = self.action_callbacks()?;
        let module = wasmtime::Module::new(&self.store.engine(), &bytes)?;
        //let instance = wasmtime::Instance::new(&mut self.store, &module, &functions)?;
        let instance = linker.instantiate(&mut self.store,&module)?;
        
        self.instance = Some(instance);

        // Initialize the module
        let start = self.get_func(function)?;
        start.call(&mut self.store,&[], &mut [])?;
        Ok(())
    }
}