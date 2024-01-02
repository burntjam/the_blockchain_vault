use crate::message::{WebSocketMessage,HelloMessage,HelloResponse,ChallengeResponse,NetworkKeyResponse,TransactionMessage};


pub trait MessageHandler {
    fn handleMessage(&self,msg: WebSocketMessage) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

pub trait MessageHandlerFactory {
    fn handleMessage(&self,msg: WebSocketMessage) -> Result<Box<dyn MessageHandler>, Box<dyn std::error::Error>>;
}