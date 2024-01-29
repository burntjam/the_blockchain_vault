use async_trait::async_trait;
use spool_client::*;
use config_lib::ChainConfig;
use std::sync::{Arc, Mutex};
use super::query_processor_manager::QueryProcessManager;

#[async_trait]
pub trait QueryConsumer {
    async fn process(&self);
}


pub struct DiskQueryConsumer {
    query_process_manager: Arc<dyn QueryProcessManager>,
    spool_manager: Arc<dyn SpoolConnectionManager>,
}

impl DiskQueryConsumer {
    pub fn new(query_process_manager: &Arc<dyn QueryProcessManager>, spool_manager: &Arc<dyn SpoolConnectionManager>) -> Result<Arc<dyn QueryConsumer>,Box<dyn std::error::Error>> {
        Ok(Arc::new(DiskQueryConsumer { query_process_manager: query_process_manager.clone(), spool_manager: spool_manager.clone() }) as Arc<dyn QueryConsumer>)
    }
}

#[async_trait]
impl QueryConsumer for DiskQueryConsumer {
    async fn process(&self) {
        let config = ChainConfig::new().unwrap();
        let spool_connection = self.spool_manager.create_connection(&rdf_store_client::RDF_STORE_TOPIC.to_string()).await.unwrap();
        while let result = spool_connection.consume().await {
            if result.is_err() {
                break;
            }
            let query_bytes = result.unwrap();
            query_bytes.iter().for_each(|entry| {
                let rdf_message: rdf_store_client::message::RdfQueryMessage = serde_json::from_str(&String::from_utf8(entry.clone()).unwrap()).unwrap();
                let processor = self.query_process_manager.createProcessor(&rdf_message).unwrap();
                tokio::spawn(async move {
                    let _ = processor.process().await;
                });
            })
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::{query_processor, MockQueryProcessManager};

    use super::*;
    use spool_client::*;
    use spool_errors::*;
    use std::sync::{Mutex, Arc,mpsc};

    struct QuerySpoolMockConnection {
        entries: Arc<Mutex<Vec<Vec<u8>>>>,
    }


    #[async_trait]
    impl spool_connection::SpoolConnection for QuerySpoolMockConnection {
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

    impl QuerySpoolMockConnection {
        pub fn new() -> Result<Arc<dyn spool_connection::SpoolConnection>, Box<dyn std::error::Error>> {
            let entry = serde_json::to_string(&rdf_store_client::message::RdfQueryMessage{client:String::from("test"),client_id:String::from("test"),query:String::from("test")}).unwrap();
            let entry2 = serde_json::to_string(&rdf_store_client::message::RdfQueryMessage{client:String::from("test"),client_id:String::from("test"),query:String::from("test")}).unwrap();
            let entry3 = serde_json::to_string(&rdf_store_client::message::RdfQueryMessage{client:String::from("test"),client_id:String::from("test"),query:String::from("test")}).unwrap();
            let entries = Arc::new(Mutex::new(vec![entry.into_bytes(),entry2.into_bytes(),entry3.into_bytes()]));
            Ok(Arc::new(QuerySpoolMockConnection {entries:entries}))
        }
    }

    struct QuerySpoolMockConnectionManager;

    impl QuerySpoolMockConnectionManager {
        pub fn new() -> Result<Arc<dyn SpoolConnectionManager>, Box<dyn std::error::Error>> {
            Ok(Arc::new(QuerySpoolMockConnectionManager{}))
        }
    }

    #[async_trait]
    impl SpoolConnectionManager for QuerySpoolMockConnectionManager {
        async fn create_connection(&self,topic: &String) -> Result<Arc<dyn SpoolConnection>, Box<dyn std::error::Error>> {
            Ok(QuerySpoolMockConnection::new()?)
        }
    }

    #[tokio::test]
    async fn test_disk_query_consumer() -> Result<(), Box<dyn std::error::Error>>{
        let query_processor_manager = MockQueryProcessManager::new()?;
        let spool_connection_manager = QuerySpoolMockConnectionManager::new()?;
        let disk_query_consumer = DiskQueryConsumer::new(&query_processor_manager, &spool_connection_manager)?;
        disk_query_consumer.process().await;
        Ok(())
    }
}

