use std::sync::Arc;
pub mod block_db_manager;
pub mod transaction_processor;
pub mod transaction_manager;
pub mod transaction_consumer;
pub mod transaction_constants;
pub mod transaction_errors;
pub mod mock;
pub mod account;
pub mod account_manager;
pub mod contract;
pub mod contract_manager;
pub mod tangle;
pub mod tangle_manager;

use block_db_manager::*;
use spool_client::SpoolGrpcClient;
use transaction_processor::*;
use transaction_manager::*;
use transaction_consumer::*;
use transaction_constants::*;
use transaction_errors::*;
use mock::*;
use account::*;
use account_manager::*;
use contract::*;
use contract_manager::*;
use tangle::*;
use tangle_manager::*;
use config_lib::*;

async fn process(transaction_consumer: Arc<dyn TransactionConsumer>) {
    println!("In the consumer");
    transaction_consumer.process().await;
}

#[tokio::main]
async fn main() {
    
    let block_db_manager = BlockDbManager::new().unwrap();
    let config = ChainConfig::new().unwrap();
    let tangle_manager = BlockTangleManager::init(block_db_manager.sessionFactory().unwrap()).unwrap();

    let processor_factory = BlockTransactionProcessorFactory::new(block_db_manager.sessionFactory().unwrap(),
        tangle_manager.clone());
    let transaction_manager = BlockTransactionManager::new(processor_factory);
    let transaction_spool_client = 
        spool_client::spool_connection_grpc::createGrpcConnection(config.block_db.spool_url, TRANSACTION_TOPIC_NAME.to_string()).await.unwrap();
    let transaction_consumer: Arc<dyn TransactionConsumer> = BlockTransactionConsumer::new(transaction_manager, transaction_spool_client);

    let transaction_processor = tokio::runtime::Runtime::new().unwrap();
    transaction_processor.spawn(async {
        process(transaction_consumer).await;
    });
    println!("Hello world");
    transaction_processor.shutdown_background();
}