use oxigraph::store::Store;
use oxigraph::model::*;
use oxigraph::sparql::QueryResults;
use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use std::fs;
use std::io;
use rdf_lib::{OxigraphStoreManager,OxigraphSessionFactory,StoreManager,StoreSessionFactory};

pub trait DbManager {
    
}

pub struct BlockDbManager {
    pub store: Arc<Mutex<Store>>,
    pub store_manager: Arc<Mutex<dyn StoreManager>>,
    pub session_factory: Arc<Mutex<dyn StoreSessionFactory>>,
}


impl DbManager for BlockDbManager {

}

impl BlockDbManager {
    pub fn new() -> Result<Arc<dyn DbManager>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new().unwrap();
        let environment = environment_lib::Environment::new();
        let path = format!("{}{}",&environment.home_directory,&config.block_db.path);
        fs::create_dir_all(path.clone())?;
        let store = Arc::new(Mutex::new(Store::open(path).unwrap()));
        let store_manager: Arc<Mutex<dyn StoreManager>> = OxigraphStoreManager::new(store.clone())?;
        let session_factory: Arc<Mutex<dyn StoreSessionFactory>> = OxigraphSessionFactory::new(store_manager.clone())?;
        Ok(Arc::new(BlockDbManager { store, store_manager, session_factory }) as Arc<dyn DbManager>)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_manager_new() {
        let blockDbManager = BlockDbManager::new();
    }
}