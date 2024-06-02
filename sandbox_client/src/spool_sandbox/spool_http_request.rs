use async_trait::async_trait;
use crate::{serialize_http_sandbox_message_wrapper,deserialize_http_sandbox_reponse_bin_message, HttpRequest, SandboxHttpMessage, SandboxHttpResponseMessage};
use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use rdf_lib::store_result_set::*;
use spool_client::{SpoolConnectionManager,SpoolConnection};


pub struct SpoolHttpRequest {
    client_name: String,
    client_id: u32,
    spool_manager: Arc<dyn SpoolConnectionManager>,
}

impl SpoolHttpRequest {
    pub fn new(client_name: &String, client_id: &u32, spool_manager: &Arc<dyn SpoolConnectionManager>) -> Result<Arc<dyn HttpRequest>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(SpoolHttpRequest {client_name: client_name.clone(), client_id: client_id.clone(), spool_manager: spool_manager.clone()}) as Arc<dyn HttpRequest>)
    }
}


#[async_trait]
impl HttpRequest for SpoolHttpRequest {
    async fn handle(&self, request: &SandboxHttpMessage) -> Result<SandboxHttpResponseMessage, Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        let sandbox_message_json = serialize_http_sandbox_message_wrapper(&crate::SandboxHttpMessageWrapper { 
            client: self.client_name.clone(), client_id: self.client_id.clone(), sandbox_http_message: request.clone() })?;
        let spool_connection = self.spool_manager.create_connection(&crate::SANDBOX_REQUEST_TOPIC.to_string()).await?;
        spool_connection.push(sandbox_message_json.into_bytes()).await?;
        if let result = spool_connection.consumeFromTopic(&format!("{}/{}",self.client_name.clone(),self.client_id)).await {
            if result.is_err() {
                return Err(result.unwrap_err());
            }
            let query_bytes = result.unwrap();
            for entry in query_bytes {
                let sandbox_http_message_wrapper = deserialize_http_sandbox_reponse_bin_message(&entry)?;
                return Ok(sandbox_http_message_wrapper);
            }
        }
        Ok(SandboxHttpResponseMessage{body:vec![]})
    }
}