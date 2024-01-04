use crate::block_db_manager::{DbManager};
use std::sync::{Arc,Mutex};
use rdf_lib::{StoreSessionFactory,MockStoreSessionFactory};

pub struct MockDbManager;

impl DbManager for MockDbManager {
    fn sessionFactory(&self) -> Result<Arc<Mutex<dyn StoreSessionFactory>>,Box<dyn std::error::Error>> {
        Ok(MockStoreSessionFactory::new()?)
    }

}

impl MockDbManager {
    pub fn new() -> Arc<dyn DbManager> {
        Arc::new(MockDbManager { }) as Arc<dyn DbManager>
    }
}