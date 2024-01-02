use crate::handler::message::*;
use crate::message::*;
use super::session::*;
use super::mock::*;
use std::sync::Arc;


pub struct NetworkKeyResposneHandler {
    session: Arc<dyn Session>,
}

impl NetworkKeyResposneHandler {
    pub fn new(session: Arc<dyn Session>) -> Self {
        NetworkKeyResposneHandler { session }
    }
}

impl MessageHandler for NetworkKeyResposneHandler {
    fn handleMessage(&self,msg: WebSocketMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_handler_creation() {
        let session = Arc::new(MockSession {});
        let handler = NetworkKeyResposneHandler::new(session);
        // Here you can add assertions if there are any properties to check
    }

    #[test]
    fn test_network_handle_message() {
        let session = Arc::new(MockSession {});
        let handler = NetworkKeyResposneHandler::new(session);

        // You'll need an instance of WebSocketMessage. The specifics depend on its definition.
        let message = WebSocketMessage::NetworkKeyResponse(
            NetworkKeyResponse { account_id: "test".to_string(), keys: vec!["test".to_string()] }); // Replace with actual variant

        assert!(handler.handleMessage(message).is_ok());
    }
}