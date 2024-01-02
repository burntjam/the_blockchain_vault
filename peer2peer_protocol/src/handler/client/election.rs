use crate::handler::message::*;
use crate::message::*;
use super::session::*;
use super::mock::*;
use std::sync::Arc;

pub struct ElectionHandler {
    session: Arc<dyn Session>,
}

impl ElectionHandler {
    pub fn new(session: Arc<dyn Session>) -> Self {
        ElectionHandler { session }
    }
}

impl MessageHandler for ElectionHandler {
    fn handleMessage(&self,msg: WebSocketMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_election_handler_creation() {
        let session = Arc::new(MockSession {});
        let handler = ElectionHandler::new(session);
        // Here you can add assertions if there are any properties to check
    }

    #[test]
    fn test_election_handle_message() {
        let session = Arc::new(MockSession {});
        let handler = ElectionHandler::new(session);

        // You'll need an instance of WebSocketMessage. The specifics depend on its definition.
        let message = WebSocketMessage::Election(ElectionMessage { account_id: "test".to_string()});

        assert!(handler.handleMessage(message).is_ok());
    }

}