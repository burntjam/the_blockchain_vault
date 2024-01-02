use crate::handler::message::*;
use crate::message::*;
use super::session::*;
use std::sync::Arc;


pub struct TransactionHandler {
    session: Arc<dyn Session>,
}

impl TransactionHandler {
    pub fn new(session: Arc<dyn Session>) -> Self {
        TransactionHandler { session }
    }
}

impl MessageHandler for TransactionHandler {
    fn handleMessage(&self,msg: WebSocketMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

        Ok(())
    }
}