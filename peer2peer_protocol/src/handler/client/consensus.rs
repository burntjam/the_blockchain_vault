use crate::handler::message::*;
use crate::message::*;
use super::session::*;
use super::mock::*;
use std::sync::Arc;


pub struct ConcensusHandler {
    session: Arc<dyn Session>,
}

impl ConcensusHandler {
    pub fn new(session: Arc<dyn Session>) -> Self {
        ConcensusHandler { session }
    }
}

impl MessageHandler for ConcensusHandler {
    fn handleMessage(&self,msg: WebSocketMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

        Ok(())
    }
}


pub struct ChallengeHandler {
    session: Arc<dyn Session>,
}

impl ChallengeHandler {
    pub fn new(session: Arc<dyn Session>) -> Self {
        ChallengeHandler { session }
    }
}

impl MessageHandler for ChallengeHandler {
    fn handleMessage(&self,msg: WebSocketMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concensus_handler_creation() {
        let session = Arc::new(MockSession {});
        let handler = ConcensusHandler::new(session);
        // Here you can add assertions if there are any properties to check
    }

    #[test]
    fn test_consensus_handle_message() {
        let session = Arc::new(MockSession {});
        let handler = ConcensusHandler::new(session);

        // You'll need an instance of WebSocketMessage. The specifics depend on its definition.
        let message = WebSocketMessage::HelloResponse(HelloResponse{account_id:"test".to_string(),seed_hash:"test".to_string()}); // Replace with actual variant

        assert!(handler.handleMessage(message).is_ok());
    }

    #[test]
    fn test_challenge_handler_creation() {
        let session = Arc::new(MockSession {});
        let handler = ChallengeHandler::new(session);
        // Here you can add assertions if there are any properties to check
    }

    #[test]
    fn test_challenge_handle_message() {
        let session = Arc::new(MockSession {});
        let handler = ChallengeHandler::new(session);

        // You'll need an instance of WebSocketMessage. The specifics depend on its definition.
        let message = WebSocketMessage::ChallengeResponse(
            ChallengeResponse{
                account_id:"test".to_string(),
                status:"test".to_string(),
                peers:vec!["test".to_string()],
                network_key_response: Option::None
            }); // Replace with actual variant

        assert!(handler.handleMessage(message).is_ok());
    }
}