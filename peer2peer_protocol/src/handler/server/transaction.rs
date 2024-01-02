use crate::handler::message::*;
use crate::message::*;
use super::session::*;
use super::mock::*;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_handler_creation() {
        let session = Arc::new(MockSession {});
        let handler = TransactionHandler::new(session);
        // Here you can add assertions if there are any properties to check
    }

    #[test]
    fn test_hello_handle_message() {
        let session = Arc::new(MockSession {});
        let handler = TransactionHandler::new(session);

        // You'll need an instance of WebSocketMessage. The specifics depend on its definition.
        let message = WebSocketMessage::Transaction(
            TransactionMessage { source_account_id: "test".to_string(), target_account_id: "test".to_string(), transaction_type: "test".to_string(), binary_transaction: vec![] });

        assert!(handler.handleMessage(message).is_ok());
    }

}