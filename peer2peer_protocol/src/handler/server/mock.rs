use super::socket::*;
use super::session::*;
use std::sync::Arc;

pub struct MockServerMessageSocket;

impl ServerMessageSocket for MockServerMessageSocket {
    fn send(&self, msg: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}

pub struct MockSession;

impl Session for MockSession {
    fn accountId(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok("test".to_string())
    }
    fn socket(&self) -> Result<Arc<dyn ServerMessageSocket>, Box<dyn std::error::Error>> {
        Ok(Arc::new(MockServerMessageSocket))
    }
}