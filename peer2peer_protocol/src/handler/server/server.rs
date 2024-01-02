use crate::handler::message::{MessageHandler,MessageHandlerFactory};
use crate::message::{WebSocketMessage,HelloMessage,HelloResponse,ChallengeResponse,NetworkKeyResponse,TransactionMessage};
use super::socket::*;
use super::consensus::*;
use super::network::*;
use super::transaction::*;
use super::heartbeat::*;
use super::election::*;
use super::session::*;
use super::mock::*;
use std::sync::Arc;


pub struct HelloMessageHandler {
    session: Arc<dyn Session>
}

impl HelloMessageHandler {
    pub fn new(session: Arc<dyn Session>) -> Self {
        HelloMessageHandler { session }
    }
}

impl MessageHandler for HelloMessageHandler {
    fn handleMessage(&self,msg: WebSocketMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.session.socket().unwrap().send(
            serde_json::to_string(
                &HelloMessage::new(
                    self.session.accountId().unwrap())
            ).unwrap())
    }
}

pub struct ServerMessageHandlerFactory {
    session: Arc<dyn Session>,
}

impl ServerMessageHandlerFactory {
    pub fn new(session: Arc<dyn Session>) -> Self {
        ServerMessageHandlerFactory { session }
    }
}

impl MessageHandlerFactory for ServerMessageHandlerFactory {
    fn handleMessage(&self,msg: WebSocketMessage) -> Result<Box<dyn MessageHandler>, Box<dyn std::error::Error>> {
        match msg {
            WebSocketMessage::Hello (message) => {
                Ok(Box::new(HelloMessageHandler::new(self.session.clone())))
            }
            WebSocketMessage::Challenge (message) => {
                Ok(Box::new(ChallengeMessageHandler::new(self.session.clone())))
            }
            WebSocketMessage::NetworkKeyQuery (message) => {
                Ok(Box::new(NetworkQueryHandler::new(self.session.clone())))
            }
            WebSocketMessage::Transaction (message) => {
                Ok(Box::new(TransactionHandler::new(self.session.clone())))
            }
            WebSocketMessage::Election (message) => {
                Ok(Box::new(ElectionHandler::new(self.session.clone())))
            }
            WebSocketMessage::NetworkHeartBeat (message) => {
                Ok(Box::new(NetworkHeartBeatHandler::new(self.session.clone())))
            }
            _ => {
                return Err("Message type is current not supported.".into());
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{ChallengeMessage, ElectionMessage, NetworkHeartBeat};
    use chrono::{DateTime, Utc};

    use super::*;

    #[test]
    fn test_hello_handler_creation() {
        let session = Arc::new(MockSession {});
        let handler = HelloMessageHandler::new(session);
        // Here you can add assertions if there are any properties to check
    }

    #[test]
    fn test_hello_handle_message() {
        let session = Arc::new(MockSession {});
        let handler = HelloMessageHandler::new(session);

        // You'll need an instance of WebSocketMessage. The specifics depend on its definition.
        let message = WebSocketMessage::Hello(HelloMessage { account_id: "test".to_string() }); // Replace with actual variant

        assert!(handler.handleMessage(message).is_ok());
    }

    #[test]
    fn test_client_message_handler_factory_hello() {
        let session = Arc::new(MockSession {});
        let factory = ServerMessageHandlerFactory::new(session);
        let message = WebSocketMessage::Hello(HelloMessage { account_id: "test".to_string() });
        let result = factory.handleMessage(message);

        assert!(result.is_ok());
    }

    #[test]
    fn test_client_message_handler_factory_challenge() {
        let session = Arc::new(MockSession {});
        let factory = ServerMessageHandlerFactory::new(session);
        let message = WebSocketMessage::Challenge(ChallengeMessage 
            { account_id: "test".to_string(), seed_hash: "test".to_string(), challenge: vec!["test".to_string()], signature: "test".to_string()});
        let result = factory.handleMessage(message);

        assert!(result.is_ok());
    }

    #[test]
    fn test_client_message_handler_factory_network_query() {
        let session = Arc::new(MockSession {});
        let factory = ServerMessageHandlerFactory::new(session);
        let message = WebSocketMessage::NetworkKeyQuery(crate::NetworkKeyQuery { account_id: "test".to_string() });
        let result = factory.handleMessage(message);

        assert!(result.is_ok());
    }

    #[test]
    fn test_client_message_handler_factory_transaction() {
        let session = Arc::new(MockSession {});
        let factory = ServerMessageHandlerFactory::new(session);
        let message = WebSocketMessage::Transaction(TransactionMessage 
            { source_account_id: "test".to_string(), target_account_id: "test".to_string(), transaction_type: "test".to_string(), binary_transaction: vec![] });
        let result = factory.handleMessage(message);

        assert!(result.is_ok());
    }

    #[test]
    fn test_client_message_handler_factory_election() {
        let session = Arc::new(MockSession {});
        let factory = ServerMessageHandlerFactory::new(session);
        let message = WebSocketMessage::Election(ElectionMessage{account_id:"test".to_string()});
        let result = factory.handleMessage(message);

        assert!(result.is_ok());
    }

    #[test]
    fn test_client_message_handler_factory_network_heartbeat() {
        let session = Arc::new(MockSession {});
        let factory = ServerMessageHandlerFactory::new(session);
        let message = WebSocketMessage::NetworkHeartBeat(NetworkHeartBeat 
            { cycle_id: "test".to_string(), timestamp: Utc::now(), network_slot: 1, network_election_slot: 1, 
            network_election_publish_slot: 1, network_confirmation_slot: 1 });
        let result = factory.handleMessage(message);

        assert!(result.is_ok());
    }

    #[test]
    fn test_client_message_handler_factory_error() {
        let session = Arc::new(MockSession {});
        let factory = ServerMessageHandlerFactory::new(session);
        let message = 
            WebSocketMessage::ChallengeResponse(ChallengeResponse 
                { account_id: "test".to_string(), status: "test".to_string(), peers: vec!["test".to_string()], 
                network_key_response: Option::None } );
        let result = factory.handleMessage(message);

        assert!(result.is_err());
    }
}