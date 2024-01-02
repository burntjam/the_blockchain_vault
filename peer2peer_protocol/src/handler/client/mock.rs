use super::connection::*;
use super::session::*;
use std::sync::Arc;

pub struct MockClientMessageConnection;

impl ClientMessageConnection for MockClientMessageConnection {
    fn send(&self, msg: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}

pub struct MockSession;

impl Session for MockSession {
    fn accountId(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok("test".to_string())
    }
    fn connection(&self) -> Result<Arc<dyn ClientMessageConnection>, Box<dyn std::error::Error>> {
        Ok(Arc::new(MockClientMessageConnection))
    }
}