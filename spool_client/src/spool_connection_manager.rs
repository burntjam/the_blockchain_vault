use crate::spool_connection::SpoolConnection;
use std::sync::Arc;
use async_trait::async_trait;

#[async_trait]
pub trait SpoolConnectionManager: Sync + Send {
    async fn create_connection(&self,topic: &String) -> Result<Arc<dyn SpoolConnection>, Box<dyn std::error::Error>>;
}