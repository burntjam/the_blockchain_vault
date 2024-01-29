use crate::spool_connection_manager::SpoolConnectionManager;
use std::sync::Arc;
use async_trait::async_trait;
use crate::spool_connection::SpoolConnection;
use crate::spool_connection_mock::SpoolMockConnection;

pub struct SpoolConnectionManagerMock {
}



impl SpoolConnectionManagerMock {
    pub async fn new() -> Result<Arc<dyn SpoolConnectionManager>, Box<dyn std::error::Error>> {
        Ok(Arc::new(SpoolConnectionManagerMock {}))
    }
}

#[async_trait]
impl SpoolConnectionManager for SpoolConnectionManagerMock {
    async fn create_connection(&self,topic: &String) -> Result<Arc<dyn SpoolConnection>, Box<dyn std::error::Error>> {
        Ok(SpoolMockConnection::new().await?)
    }
}