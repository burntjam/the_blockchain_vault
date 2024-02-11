use crate::{ActionManager,ActionExecutor, Contract, MockActionExecutor};
use std::sync::{Arc,Mutex};

pub struct MockActionManager;

impl MockActionManager {
    pub fn new() -> Result<Arc<Mutex<dyn ActionManager>>,Box<dyn std::error::Error>> {
        Ok(Arc::new(Mutex::new(MockActionManager { })))
    }
}

impl ActionManager for MockActionManager {
    fn create_action_executor(&self,contract: &Arc<Mutex<dyn Contract>>) -> Result<Arc<Mutex<dyn ActionExecutor>>,Box<dyn std::error::Error>> {
        Ok(MockActionExecutor::new()?)
    }
}