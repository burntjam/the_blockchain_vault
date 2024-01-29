mod disk_store_manager;
mod mock_store_manager;
mod query_processor;
mod query_processor_manager;
mod mock_query_processor;
mod mock_query_processor_manager;
mod query_consumer;


use config_lib::ChainConfig;
use disk_store_manager::*;
use mock_store_manager::*;
use query_processor::*;
use query_processor_manager::*;
use mock_query_processor::*;
use mock_query_processor_manager::*;
use query_consumer::*;
use spool_client::*;
use tokio::join;

#[tokio::main]
async fn main() {
    let config = ChainConfig::new().unwrap();
    let spool_manager = SpoolConnectionManagerGrpc::new(config.spool.url.clone()).await.unwrap();
    let storeManager = OxigraphDiskStoreManager::new().unwrap();
    let query_manager = DiskQueryProcessManager::new(&storeManager,&spool_manager).unwrap();
    let query_consumer = DiskQueryConsumer::new(&query_manager,&spool_manager).unwrap();
    let query_consumer_handle = query_consumer.process();

    join!(query_consumer_handle);
}