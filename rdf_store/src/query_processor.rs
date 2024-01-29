use async_trait::async_trait;
use rdf_lib::RdfResultSet;
use spool_client::*;
use config_lib::ChainConfig;
use std::sync::{Arc, Mutex};
use rdf_store_client::message::*;
use crate::DiskStoreManager;

#[async_trait]
pub trait QueryProcessor: Sync + Send {
    async fn process(&self) -> Result<(),Box<dyn std::error::Error>>;
}


pub struct DiskQueryProcessor {
    rdf_message: RdfQueryMessage,
    disk_store_manager: Arc<dyn DiskStoreManager>,
    spool_manager: Arc<dyn SpoolConnectionManager>,
}


impl DiskQueryProcessor {
    pub fn new(rdf_message: &RdfQueryMessage, disk_store_manager: &Arc<dyn DiskStoreManager>, spool_manager: &Arc<dyn SpoolConnectionManager>) -> Result<Arc<dyn QueryProcessor>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new().unwrap();
        Ok(Arc::new(DiskQueryProcessor { rdf_message: rdf_message.clone(), disk_store_manager: disk_store_manager.clone(), spool_manager: spool_manager.clone() }) as Arc<dyn QueryProcessor>)
    }
    fn processQuery(&self) -> Result<RdfResultSet,Box<dyn std::error::Error>> {
        let store_session_factory = self.disk_store_manager.get_session_factory()?;
        let store_session_factory_ref = store_session_factory.lock().unwrap();
        let session = store_session_factory_ref.createSession()?;
        let session_ref = session.lock().unwrap();
        Ok(session_ref.query(&self.rdf_message.query)?)
    }
}

#[async_trait]
impl QueryProcessor for DiskQueryProcessor {
    async fn process(&self) -> Result<(),Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        let message = rdf_store_client::message::RdfResponseMessage{
            client: self.rdf_message.client.clone(),
            client_id: self.rdf_message.client_id.clone(),
            rdf_result_set: self.processQuery()?};
        let message_str = serde_json::to_string(&message).unwrap();
        let spool_connection = self.spool_manager.create_connection(
            &format!("{}/{}",self.rdf_message.client.clone(),self.rdf_message.client_id.clone())).await?;
        let _ = spool_connection.push(message_str.into_bytes()).await;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::MockDiskStoreManager;

    use super::*;
    use spool_client::*;
    use spool_errors::*;
    use std::sync::{Mutex, Arc,mpsc};
    use std::error::Error;
    use std::fs::File;
    use std::io::BufReader;
    use oxigraph::io::{DatasetFormat, GraphFormat};
    use oxigraph::model::vocab::{rdf, xsd};
    use oxigraph::model::*;
    use oxigraph::store::Store;
    

    struct QueryProcessorSpoolMockConnection {
        entries: Arc<Mutex<Vec<Vec<u8>>>>,
    }


    #[async_trait]
    impl spool_connection::SpoolConnection for QueryProcessorSpoolMockConnection {
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

    impl QueryProcessorSpoolMockConnection {
        pub fn new() -> Result<Arc<dyn spool_connection::SpoolConnection>, Box<dyn std::error::Error>> {
            let entry = serde_json::to_string(&rdf_store_client::message::RdfQueryMessage{client:String::from("test"),client_id:String::from("test"),query:String::from("test")}).unwrap();
            let entry2 = serde_json::to_string(&rdf_store_client::message::RdfQueryMessage{client:String::from("test"),client_id:String::from("test"),query:String::from("test")}).unwrap();
            let entry3 = serde_json::to_string(&rdf_store_client::message::RdfQueryMessage{client:String::from("test"),client_id:String::from("test"),query:String::from("test")}).unwrap();
            let entries = Arc::new(Mutex::new(vec![entry.into_bytes(),entry2.into_bytes(),entry3.into_bytes()]));
            Ok(Arc::new(QueryProcessorSpoolMockConnection {entries:entries}))
        }
    }

    struct QueryProcessorSpoolMockConnectionManager;

    impl QueryProcessorSpoolMockConnectionManager {
        pub fn new() -> Result<Arc<dyn SpoolConnectionManager>, Box<dyn std::error::Error>> {
            Ok(Arc::new(QueryProcessorSpoolMockConnectionManager{}))
        }
    }

    #[async_trait]
    impl SpoolConnectionManager for QueryProcessorSpoolMockConnectionManager {
        async fn create_connection(&self,topic: &String) -> Result<Arc<dyn SpoolConnection>, Box<dyn std::error::Error>> {
            Ok(QueryProcessorSpoolMockConnection::new()?)
        }
    }

    #[tokio::test]
    async fn test_process_query() -> Result<(), Box<dyn std::error::Error>>{
        let file_path = "test/data/small_test_dataset_50_triples.ttl"; // Update with the actual path

        let spool_connection_manager = QueryProcessorSpoolMockConnectionManager::new()?;
        let disk_store_manager = MockDiskStoreManager::new()?;

        {
            // Open the file in read-only mode
            let file = File::open(file_path)?;
            let reader = BufReader::new(file);

            // Load the data from the Turtle file into the store
        
            let store = disk_store_manager.get_store()?;
            let store_ref = store.lock().unwrap();
            store_ref.load_graph(reader, GraphFormat::Turtle, &GraphName::DefaultGraph, None)?;
        }
        
        let rdf_message = rdf_store_client::message::RdfQueryMessage{client:String::from("test"),client_id:String::from("test"),
            query:String::from("SELECT ?s ?p ?o WHERE { ?s ?p ?o }")};
        
        
        println!("Before creating a new processor");
        let disk_query_processor = DiskQueryProcessor::new(&rdf_message, &disk_store_manager, &spool_connection_manager)?;

        println!("Execute the query");
        disk_query_processor.process().await?;

        println!("Complete");

        Ok(())
    }

}