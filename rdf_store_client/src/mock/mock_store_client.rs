use async_trait::async_trait;
use crate::{StoreClient};
use config_lib::ChainConfig;
use std::sync::{Arc,Mutex};
use rdf_lib::store_result_set::*;
use spool_client::{SpoolConnectionManager,SpoolConnection};

pub struct MockStoreClient;

impl MockStoreClient {
    pub fn new() -> Result<Arc<Mutex<dyn StoreClient>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(Mutex::new(MockStoreClient {})) as Arc<Mutex<dyn StoreClient>>)
    }
}

#[async_trait]
impl StoreClient for MockStoreClient {
    async fn query_async(&self, query: &String) -> Result<RdfResultSet, Box<dyn std::error::Error>> {
        Ok(RdfResultSet{column_headings:vec![],rows:vec![]})
    }
    async fn persist_signed_block(&self, signed_block: &Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {

        Ok(())
    }
}