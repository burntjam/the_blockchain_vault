use crate::{TangleManager,Tangle,MockTangle};
use peer2peer_protocol::{deserialize_bin_message, WebSocketMessage, handler::message, TransactionMessage};
use std::sync::{Arc,Mutex};


pub struct MockTangleManager;


impl TangleManager for MockTangleManager {
    fn managed_transaction(&self, transaction: &TransactionMessage) -> Result<bool,Box<dyn std::error::Error>> {
        Ok(false)
    }
    fn create_tangle(&self,account_ids: &Vec<Vec<u8>>) -> Result<Arc<Mutex<dyn Tangle>>,Box<dyn std::error::Error>> {
        Ok(MockTangle::new()?)
    }
    fn get_tangle(&self,tangle_id: &Vec<u8>) -> Result<Arc<Mutex<dyn Tangle>>,Box<dyn std::error::Error>> {
        Ok(MockTangle::new()?)
    }
    fn get_active_tangle(&self) -> Result<Option<Arc<Mutex<dyn Tangle>>>,Box<dyn std::error::Error>> {
        Ok(Option::None)
    }
    fn set_active_tangle(&mut self,tangle_id: &Vec<u8>) -> Result<Option<Arc<Mutex<dyn Tangle>>>,Box<dyn std::error::Error>> {
        Ok(Option::None)
    }

}

impl MockTangleManager {
    pub fn new() -> Result<Arc<Mutex<dyn TangleManager>>,Box<dyn std::error::Error>> {
        Ok(Arc::new(Mutex::new(MockTangleManager { })) as Arc<Mutex<dyn TangleManager>>)
    }
}