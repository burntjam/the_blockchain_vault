use async_trait::async_trait;
use rdf_lib::store_result_set::*;

#[async_trait]
pub trait StoreClient: Sync + Send {
    async fn query_async(&self, query: &String) -> Result<RdfResultSet, Box<dyn std::error::Error>>;
    async fn persist_signed_block(&self, signed_block: &Vec<u8>) -> Result<(), Box<dyn std::error::Error>>;
}



