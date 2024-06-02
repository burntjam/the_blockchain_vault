use async_trait::async_trait;
use crate::{SandboxHttpMessage,SandboxHttpResponseMessage};

#[async_trait]
pub trait HttpRequest: Sync + Send {
    async fn handle(&self, request: &SandboxHttpMessage) -> Result<SandboxHttpResponseMessage, Box<dyn std::error::Error>>;
}