use std::sync::Arc;
pub mod block_db_manager;
pub mod action_executor;
pub mod action_manager;
pub mod transaction_processor;
pub mod transaction_manager;
pub mod transaction_consumer;
pub mod transaction_constants;
pub mod transaction_errors;
pub mod mock;
pub mod account;
pub mod account_manager;
pub mod contract_errors;
pub mod contract;
pub mod contract_manager;
pub mod tangle;
pub mod tangle_manager;
pub mod producer_client;
pub mod producer_client_manager;
pub mod producer_constants;
pub mod producer_transaction_consumer;
pub mod block_tangle_producer;
pub mod block_tangle_producer_manager;

use block_db_manager::*;
use rdf_store_client::SpoolStoreClientManager;
use spool_client::{SpoolGrpcClient, SpoolConnectionManagerGrpc};
use action_executor::*;
use action_manager::*;
use transaction_processor::*;
use transaction_manager::*;
use transaction_consumer::*;
use transaction_constants::*;
use transaction_errors::*;
use mock::*;
use account::*;
use account_manager::*;
use contract_errors::*;
use contract::*;
use contract_manager::*;
use tangle::*;
use tangle_manager::*;
use producer_client::*;
use producer_client_manager::*;
use producer_constants::*;
use producer_transaction_consumer::*;
use block_tangle_producer::*;
use block_tangle_producer_manager::*;
use block_producer_client::*;
use config_lib::*;

async fn process(transaction_consumer: Arc<dyn TransactionConsumer>) {
    println!("In the consumer");
    transaction_consumer.process().await;
}

async fn process_transactions(producer_transaction_consumer: Arc<dyn ProducerTransactionConsumer>) {
    println!("In the producer consumer");
    producer_transaction_consumer.process().await;
}

async fn process_tangles(tangle_producer_manager: Arc<dyn TangleProducerManager>) {
    println!("In the tangle producer");
    tangle_producer_manager.block_processor().await;
}

#[tokio::main]
async fn main() {
    
    let block_db_manager = BlockDbManager::new().unwrap();
    let config = ChainConfig::new().unwrap();
    let tangle_manager = BlockTangleManager::init(block_db_manager.sessionFactory().unwrap()).unwrap();
    let spool_manager= SpoolConnectionManagerGrpc::new(config.spool.url).await.unwrap();
    let session_client_manager = SpoolStoreClientManager::new(&RDF_QUERY_RESPONSE_TOPIC_PREFIX.to_string(), &spool_manager).unwrap();
    let contract_manager = BlockContractManager::new( &session_client_manager).unwrap();
    let action_manager = BlockActionManager::new(&spool_manager).unwrap();
    let producer_client_manager = BlockProducerClientManager::new(&spool_manager).unwrap();
    let transaction_processor_client_manager = BlockTransactionProcessorClientManager::new(&spool_manager);
    let processor_factory = BlockTransactionProcessorFactory::new(&block_db_manager.sessionFactory().unwrap(),
        &tangle_manager,&contract_manager, &action_manager, &producer_client_manager, &transaction_processor_client_manager);
    let transaction_manager = BlockTransactionManager::new(processor_factory);
    let transaction_spool_client = 
        spool_client::spool_connection_grpc::createGrpcConnection(config.block_db.spool_url.clone(), TRANSACTION_TOPIC_NAME.to_string()).await.unwrap();
    let producer_transaction_spool_client = 
        spool_client::spool_connection_grpc::createGrpcConnection(config.block_db.spool_url.clone(), BLOCK_PRODUCER_TRANSACTION_SUBMISSION.to_string()).await.unwrap();
    let transaction_consumer: Arc<dyn TransactionConsumer> = BlockTransactionConsumer::new(transaction_manager, transaction_spool_client);
    let tangle_producer_manager = BlockTangleProducerManager::new(&block_db_manager,&session_client_manager);
    let producer_transaction_consumer = BlockProducerTransactionConsumer::new(&producer_transaction_spool_client,&tangle_producer_manager);

    let transaction_processor = tokio::runtime::Runtime::new().unwrap();
    transaction_processor.spawn(async {
        process(transaction_consumer).await;
    });
    let producer_transaction_processor = tokio::runtime::Runtime::new().unwrap();
    producer_transaction_processor.spawn(async {
        process_transactions(producer_transaction_consumer).await;
    });
    let tangle_processor = tokio::runtime::Runtime::new().unwrap();
    tangle_processor.spawn(async {
        process_tangles(tangle_producer_manager).await;
    });
    println!("Hello world");
    producer_transaction_processor.shutdown_background();
    transaction_processor.shutdown_background();
}