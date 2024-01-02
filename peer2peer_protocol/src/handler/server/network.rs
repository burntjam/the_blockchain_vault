use crate::handler::message::*;
use crate::message::*;
use super::session::*;
use super::mock::*;
use std::ptr::null;
use std::sync::Arc;


pub struct NetworkQueryHandler {
    session: Arc<dyn Session>,
}

impl NetworkQueryHandler {
    pub fn new(session: Arc<dyn Session>) -> Self {
        NetworkQueryHandler { session }
    }
}

impl MessageHandler for NetworkQueryHandler {
    fn handleMessage(&self,msg: WebSocketMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_handler_creation() {
        let session = Arc::new(MockSession {});
        let handler = NetworkQueryHandler::new(session);
        // Here you can add assertions if there are any properties to check
    }

    #[test]
    fn test_hello_handle_message() {
        let session = Arc::new(MockSession {});
        let handler = NetworkQueryHandler::new(session);

        // You'll need an instance of WebSocketMessage. The specifics depend on its definition.
        let message = WebSocketMessage::NetworkKeyQuery(NetworkKeyQuery{account_id:"test".to_string()});

        assert!(handler.handleMessage(message).is_ok());
    }

}