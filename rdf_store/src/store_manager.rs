use oxigraph::store::Store;
use oxigraph::model::*;
use oxigraph::sparql::QueryResults;
use config_lib::ChainConfig;


pub struct StoreManager {
    store: Store,
}

impl StoreManager {
    pub fn new() -> Self {
        let config = ChainConfig::new().unwrap();
        let environment = environment_lib::Environment::new();
        let path = format!("{}{}",&environment.home_directory,&config.rdf.path);
        let store = Store::open(path).unwrap();
        StoreManager { store:store }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_manager_new() {
        let storeManager = StoreManager::new();
    }
}