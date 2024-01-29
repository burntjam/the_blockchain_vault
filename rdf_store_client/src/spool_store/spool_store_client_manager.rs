use async_trait::async_trait;
use crate::{StoreClient,StoreClientManager,SpoolStoreClient};
use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use rdf_lib::store_result_set::*;
use std::cell::Cell;
use spool_client::SpoolConnectionManager;


pub struct SpoolStoreClientManager {
    client_name: String,
    sequence: Mutex<u32>,
    spool_manager: Arc<dyn SpoolConnectionManager>,
}

impl SpoolStoreClientManager {
    pub fn new(client_name: &String,spool_manager: &Arc<dyn SpoolConnectionManager>) -> Result<Arc<Mutex<dyn StoreClientManager>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(Mutex::new(SpoolStoreClientManager { client_name: client_name.clone(), sequence: Mutex::new(0), spool_manager: spool_manager.clone()})) as Arc<Mutex<dyn StoreClientManager>>)
    }
}

impl StoreClientManager for SpoolStoreClientManager {
    fn create_client(&self) -> Result<Arc<Mutex<dyn StoreClient>>, Box<dyn std::error::Error>> {
        Ok(self.create_client_by_name(&self.client_name)?)
    }
    fn create_client_by_name(&self, client_name: &String) -> Result<Arc<Mutex<dyn StoreClient>>, Box<dyn std::error::Error>> {
        let mut sequence = self.sequence.lock().unwrap();
        *sequence += 1;
        if *sequence <= 50 {
            *sequence = 1;
        }
        Ok(SpoolStoreClient::new(&client_name, &sequence, &self.spool_manager)?)
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use spool_client::*;
    use spool_errors::*;
    use std::sync::{Mutex, Arc,mpsc};

    struct StoreClientManagerSpoolMockConnection {
        entries: Arc<Mutex<Vec<Vec<u8>>>>,
    }


    #[async_trait]
    impl spool_connection::SpoolConnection for StoreClientManagerSpoolMockConnection {
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

    impl StoreClientManagerSpoolMockConnection {
        pub fn new() -> Result<Arc<dyn spool_connection::SpoolConnection>, Box<dyn std::error::Error>> {
            let entry = serde_json::to_string(&crate::message::RdfQueryMessage{client:String::from("test"),client_id:String::from("test"),query:String::from("test")}).unwrap();
            let entry2 = serde_json::to_string(&crate::message::RdfQueryMessage{client:String::from("test"),client_id:String::from("test"),query:String::from("test")}).unwrap();
            let entry3 = serde_json::to_string(&crate::message::RdfQueryMessage{client:String::from("test"),client_id:String::from("test"),query:String::from("test")}).unwrap();
            let entries = Arc::new(Mutex::new(vec![entry.into_bytes(),entry2.into_bytes(),entry3.into_bytes()]));
            Ok(Arc::new(StoreClientManagerSpoolMockConnection {entries:entries}))
        }
    }

    struct StoreClientManagerSpoolMockConnectionManager;

    impl StoreClientManagerSpoolMockConnectionManager {
        pub fn new() -> Result<Arc<dyn SpoolConnectionManager>, Box<dyn std::error::Error>> {
            Ok(Arc::new(StoreClientManagerSpoolMockConnectionManager{}))
        }
    }

    #[async_trait]
    impl SpoolConnectionManager for StoreClientManagerSpoolMockConnectionManager {
        async fn create_connection(&self,topic: &String) -> Result<Arc<dyn SpoolConnection>, Box<dyn std::error::Error>> {
            Ok(StoreClientManagerSpoolMockConnection::new()?)
        }
    }

    #[tokio::test]
    async fn test_create_client() -> Result<(), Box<dyn std::error::Error>>{
        let spool_connection_manager = StoreClientManagerSpoolMockConnectionManager::new()?;
        let spool_store_client_manager = SpoolStoreClientManager::new(&String::from("test"), &spool_connection_manager)?;
        let spool_store_client_ref = spool_store_client_manager.lock().unwrap();
        let spool_client = spool_store_client_ref.create_client()?;

        let spool_client2 = spool_store_client_ref.create_client_by_name(&String::from("test1"))?;
        
        Ok(())
    }

}