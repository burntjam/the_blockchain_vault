use crate::handler::message::*;
use crate::message::*;
use super::session::*;
use super::mock::*;
use std::sync::Arc;
use chrono::{DateTime, Utc};

pub struct NetworkHeartBeatHandler {
    session: Arc<dyn Session>,
}

impl NetworkHeartBeatHandler {
    pub fn new(session: Arc<dyn Session>) -> Self {
        NetworkHeartBeatHandler { session }
    }
}

impl MessageHandler for NetworkHeartBeatHandler {
    fn handleMessage(&self,msg: WebSocketMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_heart_beat_handler_creation() {
        let session = Arc::new(MockSession {});
        let handler = NetworkHeartBeatHandler::new(session);
        // Here you can add assertions if there are any properties to check
    }

    #[test]
    fn test_network_heart_beat_handle_message() {
        let session = Arc::new(MockSession {});
        let handler = NetworkHeartBeatHandler::new(session);

        // You'll need an instance of WebSocketMessage. The specifics depend on its definition.
        let message = WebSocketMessage::NetworkHeartBeat(NetworkHeartBeat 
            { cycle_id: "test".to_string(), timestamp: Utc::now(), network_slot: 1, network_election_slot: 1, network_election_publish_slot: 1, network_confirmation_slot: 1 });

        assert!(handler.handleMessage(message).is_ok());
    }

}