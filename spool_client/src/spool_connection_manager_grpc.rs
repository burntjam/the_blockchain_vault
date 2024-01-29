use crate::spool_connection_manager::SpoolConnectionManager;
use std::sync::Arc;
use async_trait::async_trait;
use crate::spool_connection::SpoolConnection;
use crate::spool_connection_grpc::SpoolGrpcConnection;

pub struct SpoolConnectionManagerGrpc {
    url: String,
}



impl SpoolConnectionManagerGrpc {
    pub async fn new(url: String) -> Result<Arc<dyn SpoolConnectionManager>, Box<dyn std::error::Error>> {
        Ok(Arc::new(SpoolConnectionManagerGrpc {
            url,
        }))
    }
}

#[async_trait]
impl SpoolConnectionManager for SpoolConnectionManagerGrpc {
    async fn create_connection(&self,topic: &String) -> Result<Arc<dyn SpoolConnection>, Box<dyn std::error::Error>> {
        Ok(SpoolGrpcConnection::new(&self.url,topic).await?)
    }
}