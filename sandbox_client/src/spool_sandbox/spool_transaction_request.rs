use async_trait::async_trait;
use crate::{deserialize_sandbox_reponse_bin_message, serialize_sandbox_message_wrapper, serialize_sandbox_message, SandboxMessage, SandboxResponseMessage, TransactionRequest};
use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use rdf_lib::store_result_set::*;
use spool_client::{SpoolConnectionManager,SpoolConnection};


pub struct SpoolTransactionRequest {
    client_name: String,
    client_id: u32,
    spool_manager: Arc<dyn SpoolConnectionManager>,
}

impl SpoolTransactionRequest {
    pub fn new(client_name: &String, client_id: &u32, spool_manager: &Arc<dyn SpoolConnectionManager>) -> Result<Arc<dyn TransactionRequest>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(SpoolTransactionRequest {client_name: client_name.clone(), client_id: client_id.clone(), spool_manager: spool_manager.clone()}) as Arc<dyn TransactionRequest>)
    }
}

#[async_trait]
impl TransactionRequest for SpoolTransactionRequest {
    async fn handle(&self, request: &SandboxMessage) -> Result<SandboxResponseMessage, Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        let sandbox_message_json = serialize_sandbox_message_wrapper(&crate::SandboxMessageWrapper { 
            client: self.client_name.clone(), client_id: self.client_id.clone(), sandbox_message: request.clone() })?;
        let spool_connection = self.spool_manager.create_connection(&crate::SANDBOX_REQUEST_TOPIC.to_string()).await?;
        spool_connection.push(sandbox_message_json.into_bytes()).await?;
        if let result = spool_connection.consumeFromTopic(&format!("{}/{}",self.client_name.clone(),self.client_id)).await {
            if result.is_err() {
                return Err(result.unwrap_err());
            }
            let query_bytes = result.unwrap();
            for entry in query_bytes {
                let sandbox_message_wrapper = deserialize_sandbox_reponse_bin_message(&entry)?;
                return Ok(sandbox_message_wrapper);
            }
        }
        Ok(SandboxResponseMessage{contract:vec![],action_type: String::from("value"),change_set:vec![]})
    }
}