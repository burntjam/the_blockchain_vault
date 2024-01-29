use async_trait::async_trait;
use crate::{StoreClient};
use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use rdf_lib::store_result_set::*;
use spool_client::{SpoolConnectionManager,SpoolConnection};

pub struct SpoolStoreClient {
    client_name: String,
    client_id: u32,
    spool_manager: Arc<dyn SpoolConnectionManager>,
}

impl SpoolStoreClient {
    pub fn new(client_name: &String, client_id: &u32, spool_manager: &Arc<dyn SpoolConnectionManager>) -> Result<Arc<Mutex<dyn StoreClient>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(Mutex::new(SpoolStoreClient { client_name: client_name.clone(), client_id: client_id.clone(), spool_manager: spool_manager.clone()})) as Arc<Mutex<dyn StoreClient>>)
    }
}

#[async_trait]
impl StoreClient for SpoolStoreClient {
    async fn query_async(&self, query: &String) -> Result<RdfResultSet, Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        let rdf_query_message = serde_json::to_string(&crate::message::RdfQueryMessage{
            client:self.client_name.clone(),client_id:self.client_id.to_string().clone(),query:query.clone()}).unwrap();
        let spool_connection = self.spool_manager.create_connection(&crate::RDF_STORE_TOPIC.to_string()).await?;
        spool_connection.push(rdf_query_message.into_bytes()).await?;
        if let result = spool_connection.consumeFromTopic(&format!("{}/{}",self.client_name.clone(),self.client_id)).await {
            if result.is_err() {
                return Err(result.unwrap_err());
            }
            let query_bytes = result.unwrap();
            for entry in query_bytes {
                let rdf_message: crate::message::RdfResponseMessage = serde_json::from_str(&String::from_utf8(entry.clone()).unwrap()).unwrap();
                return Ok(rdf_message.rdf_result_set.clone());
            }
        }
        Ok(RdfResultSet{column_headings:vec![],rows:vec![]})
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use spool_client::*;
    use spool_errors::*;
    use std::sync::{Mutex, Arc,mpsc};

    struct StoreClientSpoolMockConnection {
        entries: Arc<Mutex<Vec<Vec<u8>>>>,
    }


    #[async_trait]
    impl spool_connection::SpoolConnection for StoreClientSpoolMockConnection {
        async fn push(&self,message: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
            
            Ok(())
        }

        async fn pushToTopic(&self,message: Vec<u8>, topic: &String) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
        async fn consume(&self) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
            let mut transactions = self.entries.lock().unwrap();
            if transactions.len() > 0 {
                let result = transactions.clone();
                transactions.truncate(0);
                return Ok(result);
            }
            Err(Box::new(spool_errors::SpoolDisconnectError { message: "An error occurred".to_string() }))
        }

        async fn consumeFromTopic(&self, topic: &String) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
            let mut transactions = self.entries.lock().unwrap();
            if transactions.len() > 0 {
                let result = transactions.clone();
                transactions.truncate(0);
                return Ok(result);
            }
            Err(Box::new(spool_errors::SpoolDisconnectError { message: "An error occurred".to_string() }))
        }
    }

    impl StoreClientSpoolMockConnection {
        pub fn new() -> Result<Arc<dyn spool_connection::SpoolConnection>, Box<dyn std::error::Error>> {
            let entry = serde_json::to_string(&crate::message::RdfResponseMessage{client:String::from("test"),client_id:String::from("test"),rdf_result_set:RdfResultSet{column_headings:vec![],rows:vec![]}}).unwrap();
            let entries = Arc::new(Mutex::new(vec![entry.into_bytes()]));
            Ok(Arc::new(StoreClientSpoolMockConnection {entries:entries}))
        }
    }

    struct StoreClientSpoolMockConnectionManager;

    impl StoreClientSpoolMockConnectionManager {
        pub fn new() -> Result<Arc<dyn SpoolConnectionManager>, Box<dyn std::error::Error>> {
            Ok(Arc::new(StoreClientSpoolMockConnectionManager{}))
        }
    }

    #[async_trait]
    impl SpoolConnectionManager for StoreClientSpoolMockConnectionManager {
        async fn create_connection(&self,topic: &String) -> Result<Arc<dyn SpoolConnection>, Box<dyn std::error::Error>> {
            Ok(StoreClientSpoolMockConnection::new()?)
        }
    }

    #[tokio::test]
    async fn test_spool_client() -> Result<(), Box<dyn std::error::Error>>{
        let spool_connection_manager = StoreClientSpoolMockConnectionManager::new()?;
        let spool_store_client = SpoolStoreClient::new(&String::from("test"), &1, &spool_connection_manager)?;
        let spool_store_client_ref = spool_store_client.lock().unwrap();
        let spool_client = spool_store_client_ref.query_async(&String::from("SELECT ?s ?p ?o WHERE { ?s ?p ?o }")).await?;        
        Ok(())
    }

}