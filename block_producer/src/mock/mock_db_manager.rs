use crate::block_db_manager::{DbManager};
use std::sync::Arc;

pub struct MockDbManager;

impl DbManager for MockDbManager {
}

impl MockDbManager {
    pub fn new() -> Arc<dyn DbManager> {
        Arc::new(MockDbManager { }) as Arc<dyn DbManager>
    }
}