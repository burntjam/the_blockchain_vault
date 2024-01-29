use oxigraph::store::Store;
use oxigraph::model::*;
use oxigraph::sparql::QueryResults;
use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use std::fs;
use std::io;
use rdf_lib::{OxigraphStoreManager,OxigraphSessionFactory,StoreManager,StoreSessionFactory};
use crate::disk_store_manager::DiskStoreManager;

pub struct MockDiskStoreManager {
    pub store: Arc<Mutex<Store>>,
    pub store_manager: Arc<Mutex<dyn StoreManager>>,
    pub session_factory: Arc<Mutex<dyn StoreSessionFactory>>,
}

impl MockDiskStoreManager {
    pub fn new() -> Result<Arc<dyn DiskStoreManager>,Box<dyn std::error::Error>> {
        let store = Arc::new(Mutex::new(Store::new().unwrap()));
        let store_manager: Arc<Mutex<dyn StoreManager>> = OxigraphStoreManager::new(store.clone())?;
        let session_factory: Arc<Mutex<dyn StoreSessionFactory>> = OxigraphSessionFactory::new(store_manager.clone())?;
        Ok(Arc::new(MockDiskStoreManager{store, store_manager, session_factory}))
    }
}

impl DiskStoreManager for MockDiskStoreManager {
    fn get_store(&self) -> Result<Arc<Mutex<Store>>,Box<dyn std::error::Error>> {
        Ok(self.store.clone())
    }
    fn get_store_manager(&self) -> Result<Arc<Mutex<dyn StoreManager>>,Box<dyn std::error::Error>> {
        Ok(self.store_manager.clone())
    }
    fn get_session_factory(&self) -> Result<Arc<Mutex<dyn StoreSessionFactory>>,Box<dyn std::error::Error>> {
        Ok(self.session_factory.clone())
    }
}


