use async_trait::async_trait;
use crate::{SandboxMessage,SandboxResponseMessage};

#[async_trait]
pub trait TransactionRequest: Sync + Send {
    async fn handle(&self, request: &SandboxMessage) -> Result<SandboxResponseMessage, Box<dyn std::error::Error>>;
}