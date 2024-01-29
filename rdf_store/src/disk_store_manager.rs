use oxigraph::store::Store;
use oxigraph::model::*;
use oxigraph::sparql::QueryResults;
use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use std::fs;
use std::io;
use rdf_lib::{OxigraphStoreManager,OxigraphSessionFactory,StoreManager,StoreSessionFactory};

pub trait DiskStoreManager: Sync + Send {
    fn get_store(&self) -> Result<Arc<Mutex<Store>>,Box<dyn std::error::Error>>;
    fn get_store_manager(&self) -> Result<Arc<Mutex<dyn StoreManager>>,Box<dyn std::error::Error>>;
    fn get_session_factory(&self) -> Result<Arc<Mutex<dyn StoreSessionFactory>>,Box<dyn std::error::Error>>;
}

pub struct OxigraphDiskStoreManager {
    pub store: Arc<Mutex<Store>>,
    pub store_manager: Arc<Mutex<dyn StoreManager>>,
    pub session_factory: Arc<Mutex<dyn StoreSessionFactory>>,
}

impl OxigraphDiskStoreManager {
    pub fn new() -> Result<Arc<dyn DiskStoreManager>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new().unwrap();
        let environment = environment_lib::Environment::new();
        let path = format!("{}{}",&environment.home_directory,&config.rdf.path);
        fs::create_dir_all(path.clone())?;
        let store = Arc::new(Mutex::new(Store::open(path).unwrap()));
        let store_manager: Arc<Mutex<dyn StoreManager>> = OxigraphStoreManager::new(store.clone())?;
        let session_factory: Arc<Mutex<dyn StoreSessionFactory>> = OxigraphSessionFactory::new(store_manager.clone())?;
        Ok(Arc::new(OxigraphDiskStoreManager { store, store_manager, session_factory }))
    }
}

impl DiskStoreManager for OxigraphDiskStoreManager {
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_manager_new() {
        let storeManager = OxigraphDiskStoreManager::new().unwrap();
    }
}