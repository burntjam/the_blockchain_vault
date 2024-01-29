use crate::Tangle;
use peer2peer_protocol::{deserialize_bin_message, WebSocketMessage, handler::message, TransactionMessage};
use std::sync::{Arc,Mutex};


pub struct MockTangle;

impl Tangle for MockTangle {
    fn tangle_id(&self) -> Result<Vec<u8>,Box<dyn std::error::Error>> {
        Ok(Vec::new())
    }
    fn tangle_accounts(&self) -> Result<Vec<Vec<u8>>,Box<dyn std::error::Error>> {
        Ok(Vec::new())
    }
    fn add_account(&self,account_id: Vec<u8>) -> Result<(),Box<dyn std::error::Error>> {
        Ok(())
    }
    fn verify_account(&self,account_id: Vec<u8>) -> Result<bool,Box<dyn std::error::Error>> {
        Ok(true)
    }
}

impl MockTangle {
    pub fn new() -> Result<Arc<Mutex<dyn Tangle>>,Box<dyn std::error::Error>> {
        Ok(Arc::new(Mutex::new(MockTangle {})) as Arc<Mutex<dyn Tangle>>)
    }
}