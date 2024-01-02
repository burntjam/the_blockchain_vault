use super::connection::*;
use std::sync::Arc;

pub trait Session {
    fn accountId(&self) -> Result<String, Box<dyn std::error::Error>>;
    fn connection(&self) -> Result<Arc<dyn ClientMessageConnection>, Box<dyn std::error::Error>>;
}


pub struct BasicSession {
    account_id: String,
    client_message_connection: Arc<dyn ClientMessageConnection>,
}

impl BasicSession {
    pub fn new(acount: String, client_message_connection: Arc<dyn ClientMessageConnection>) -> Self {
        BasicSession { account_id: acount, client_message_connection: client_message_connection }
    }
}

impl Session for BasicSession {
    fn accountId(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.account_id.clone())
    }
    fn connection(&self) -> Result<Arc<dyn ClientMessageConnection>, Box<dyn std::error::Error>> {
        Ok(self.client_message_connection.clone())
    }
}

