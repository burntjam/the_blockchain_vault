use std::sync::{Mutex,Arc};
use crate::{StoreSessionFactory, StoreManager, OxigraphStoreSession, StoreSession};


pub struct OxigraphSessionFactory {
    store_manager: Arc<Mutex<dyn StoreManager>>,
}

impl OxigraphSessionFactory {
    pub fn new(store_manager: Arc<Mutex<dyn StoreManager>>) -> Result<Arc<Mutex<dyn StoreSessionFactory>>, Box<dyn std::error::Error>> {
        Ok(Arc::new(Mutex::new(
            OxigraphSessionFactory{store_manager})))
    }
}

impl StoreSessionFactory for OxigraphSessionFactory {
    fn createSession(&self) -> Result<Box<Mutex<dyn StoreSession>>, Box<dyn std::error::Error>> {
        let state = self.store_manager.lock().unwrap().createOrRetrieveState()?;
        let store_session = OxigraphStoreSession::new(state)?;
        Ok(store_session)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use oxigraph::store::Store;
    use std::error::Error;
    use crate::{OxigraphStoreManager};

    #[test]
    fn test_store_session_factory() -> Result<(), Box<dyn Error>> {
        let store = Arc::new(Mutex::new(Store::new()?));

        let store_manager = OxigraphStoreManager::new(store.clone())?;

        let factory = OxigraphSessionFactory::new(store_manager)?;

        let factory_ref = factory.lock().unwrap();
        let session = factory_ref.createSession()?;

        Ok(())
    }

}