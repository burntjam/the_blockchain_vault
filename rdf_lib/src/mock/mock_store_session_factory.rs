use std::sync::{Mutex,Arc};
use crate::{StoreSessionFactory, StoreManager, StoreSession, MockStoreSession};

pub struct MockStoreSessionFactory;

impl MockStoreSessionFactory {
    pub fn new() -> Result<Arc<Mutex<dyn StoreSessionFactory>>, Box<dyn std::error::Error>> {
        Ok(Arc::new(Mutex::new(
            MockStoreSessionFactory{})))
    }
}


impl StoreSessionFactory for MockStoreSessionFactory {
    fn createSession(&self) -> Result<Box<Mutex<dyn StoreSession>>, Box<dyn std::error::Error>> {
        Ok(MockStoreSession::new()?)
    }
}

