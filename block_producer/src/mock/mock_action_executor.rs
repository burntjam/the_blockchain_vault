use crate::{ActionExecutor, Contract};
use std::sync::{Arc,Mutex};

pub struct MockActionExecutor;

impl MockActionExecutor {
    pub fn new() -> Result<Arc<Mutex<dyn ActionExecutor>>,Box<dyn std::error::Error>> {
        Ok(Arc::new(Mutex::new(MockActionExecutor { })))
    }
}

impl ActionExecutor for MockActionExecutor {
    fn execute(&self, transaction_state: &String, change_set: &mut asn1_lib::ChangeSet) -> Result<(),Box<dyn std::error::Error>> {
        Ok(())
    }
}
