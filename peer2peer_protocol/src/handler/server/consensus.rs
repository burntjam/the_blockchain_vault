use crate::handler::message::*;
use crate::message::*;
use super::session::*;
use super::mock::*;
use std::sync::Arc;

pub struct ChallengeMessageHandler {
    session: Arc<dyn Session>,
}

impl ChallengeMessageHandler {
    pub fn new(session: Arc<dyn Session>) -> Self {
        ChallengeMessageHandler { session }
    }
}

impl MessageHandler for ChallengeMessageHandler {
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
        let handler = ChallengeMessageHandler::new(session);
        // Here you can add assertions if there are any properties to check
    }

    #[test]
    fn test_hello_handle_message() {
        let session = Arc::new(MockSession {});
        let handler = ChallengeMessageHandler::new(session);

        // You'll need an instance of WebSocketMessage. The specifics depend on its definition.
        let message = WebSocketMessage::Challenge(ChallengeMessage 
            { account_id: "test".to_string(), seed_hash: "test".to_string(), challenge: vec!["test".to_string()], signature: "test".to_string() });

        assert!(handler.handleMessage(message).is_ok());
    }

}