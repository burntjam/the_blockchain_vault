use crate::store_result_set::*;
use oxigraph::model::*;
use oxigraph::sparql::*;
use oxigraph::store::{StorageError,Store};
use oxigraph::model::vocab::{rdf, xsd};
use thread_local::ThreadLocal;
use std::cell::RefCell;
use std::sync::{Mutex,Arc};
use crate::{OxigraphStoreState,StoreState, StoreManager};

pub struct OxigraphStoreManager {
    store: Arc<Mutex<Store>>,
    oxigraph_store_state: ThreadLocal<Arc<Mutex<dyn StoreState>>>,
}

impl OxigraphStoreManager {
    pub fn new(store: Arc<Mutex<Store>>) -> Result<Arc<Mutex<dyn StoreManager>>, Box<dyn std::error::Error>> {
        Ok(Arc::new(Mutex::new(
            OxigraphStoreManager{
                store,oxigraph_store_state: ThreadLocal::new()})))
    }
}


impl StoreManager for OxigraphStoreManager {

    fn createOrRetrieveState(&self) -> Result<Arc<Mutex<dyn StoreState>>, Box<dyn std::error::Error>> {
        let result = self.oxigraph_store_state.get_or(||
            OxigraphStoreState::new(self.store.clone()).unwrap()).clone();
        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use oxigraph::store::Store;
    use std::error::Error;

    #[test]
    fn test_store_manager_create_or_retrieve() -> Result<(), Box<dyn Error>> {
        let store = Arc::new(Mutex::new(Store::new()?));

        let manager = OxigraphStoreManager::new(store.clone())?;

        let mut manager_ref = manager.lock().unwrap();

        let store_state = manager_ref.createOrRetrieveState()?;

        Ok(())
    }

}