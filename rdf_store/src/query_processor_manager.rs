use async_trait::async_trait;
use spool_client::*;
use config_lib::ChainConfig;
use std::sync::{Arc, Mutex};
use rdf_store_client::message::*;
use crate::DiskStoreManager;
use crate::query_processor::*;


pub trait QueryProcessManager: Sync + Send {
    fn createProcessor(&self,rdf_message: &RdfQueryMessage) -> Result<Arc<dyn QueryProcessor>,Box<dyn std::error::Error>>;
}

pub struct DiskQueryProcessManager {
    disk_store_manager: Arc<dyn DiskStoreManager>,
    spool_manager: Arc<dyn SpoolConnectionManager>,
}

impl DiskQueryProcessManager {
    pub fn new(disk_store_manager: &Arc<dyn DiskStoreManager>, spool_manager: &Arc<dyn SpoolConnectionManager>) -> Result<Arc<dyn QueryProcessManager>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new().unwrap();
        Ok(Arc::new(DiskQueryProcessManager { disk_store_manager: disk_store_manager.clone(), spool_manager: spool_manager.clone() }) as Arc<dyn QueryProcessManager>)
    }
}

impl QueryProcessManager for DiskQueryProcessManager {
    fn createProcessor(&self, rdf_message: &RdfQueryMessage) -> Result<Arc<dyn QueryProcessor>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(DiskQueryProcessor::new(rdf_message, &self.disk_store_manager, &self.spool_manager)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::MockDiskStoreManager;

    use super::*;
    use spool_client::*;
    use spool_errors::*;
    use std::sync::{Mutex, Arc,mpsc};

    struct QueryProcessorManagerSpoolMockConnection {
        entries: Arc<Mutex<Vec<Vec<u8>>>>,
    }


    #[async_trait]
    impl spool_connection::SpoolConnection for QueryProcessorManagerSpoolMockConnection {
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

    impl QueryProcessorManagerSpoolMockConnection {
        pub fn new() -> Result<Arc<dyn spool_connection::SpoolConnection>, Box<dyn std::error::Error>> {
            let entry = serde_json::to_string(&rdf_store_client::message::RdfQueryMessage{client:String::from("test"),client_id:String::from("test"),query:String::from("test")}).unwrap();
            let entry2 = serde_json::to_string(&rdf_store_client::message::RdfQueryMessage{client:String::from("test"),client_id:String::from("test"),query:String::from("test")}).unwrap();
            let entry3 = serde_json::to_string(&rdf_store_client::message::RdfQueryMessage{client:String::from("test"),client_id:String::from("test"),query:String::from("test")}).unwrap();
            let entries = Arc::new(Mutex::new(vec![entry.into_bytes(),entry2.into_bytes(),entry3.into_bytes()]));
            Ok(Arc::new(QueryProcessorManagerSpoolMockConnection {entries:entries}))
        }
    }

    struct QueryProcessorManagerSpoolMockConnectionManager;

    impl QueryProcessorManagerSpoolMockConnectionManager {
        pub fn new() -> Result<Arc<dyn SpoolConnectionManager>, Box<dyn std::error::Error>> {
            Ok(Arc::new(QueryProcessorManagerSpoolMockConnectionManager{}))
        }
    }

    #[async_trait]
    impl SpoolConnectionManager for QueryProcessorManagerSpoolMockConnectionManager {
        async fn create_connection(&self,topic: &String) -> Result<Arc<dyn SpoolConnection>, Box<dyn std::error::Error>> {
            Ok(QueryProcessorManagerSpoolMockConnection::new()?)
        }
    }

    #[tokio::test]
    async fn test_create_process() -> Result<(), Box<dyn std::error::Error>>{
        let spool_connection_manager = QueryProcessorManagerSpoolMockConnectionManager::new()?;
        let disk_store_manager = MockDiskStoreManager::new()?;
        let disk_query_process_manager = DiskQueryProcessManager::new(&disk_store_manager, &spool_connection_manager)?;
        let rdf_message = rdf_store_client::message::RdfQueryMessage{client:String::from("test"),client_id:String::from("test"),query:String::from("test")};
        disk_query_process_manager.createProcessor(&rdf_message)?;
        Ok(())
    }
}