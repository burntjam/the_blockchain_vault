use super::socket::*;
use std::sync::Arc;

pub trait Session {
    fn accountId(&self) -> Result<String, Box<dyn std::error::Error>>;
    fn socket(&self) -> Result<Arc<dyn ServerMessageSocket>, Box<dyn std::error::Error>>;
}


pub struct BasicServerSession {
    account_id: String,
    server_message_socket: Arc<dyn ServerMessageSocket>,
}

impl BasicServerSession {
    pub fn new(acount: String, server_message_socket: Arc<dyn ServerMessageSocket>) -> Self {
        BasicServerSession { account_id: acount, server_message_socket: server_message_socket }
    }
}

impl Session for BasicServerSession {
    fn accountId(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.account_id.clone())
    }
    fn socket(&self) -> Result<Arc<dyn ServerMessageSocket>, Box<dyn std::error::Error>> {
        Ok(self.server_message_socket.clone())
    }
}

