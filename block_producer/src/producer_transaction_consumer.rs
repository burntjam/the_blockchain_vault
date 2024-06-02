use std::sync::Arc;
use asn1_lib::TransactionMessage;
use config_lib::ChainConfig;
use spool_client::{SpoolConnection};
use crate::TangleProducerManager;

use super::mock::{MockTransactionManager};
use super::transaction_constants::*;
use super::producer_constants::*;
use super::transaction_manager::*;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;


#[async_trait]
pub trait ProducerTransactionConsumer : Sync + Send{
    async fn process(&self);
}

pub struct BlockProducerTransactionConsumer {
    spool_client: Arc<dyn SpoolConnection>,
    tangle_producer_manager: Arc<dyn TangleProducerManager>,
}


#[async_trait]
impl ProducerTransactionConsumer for BlockProducerTransactionConsumer {
    async fn process(&self) {
        while let result = self.spool_client.consumeFromTopic(&BLOCK_PRODUCER_TRANSACTION_SUBMISSION.to_string()).await {
            if result.is_err() {
                break;
            }
            let transactions = result.unwrap();
            
            transactions.iter().for_each(|entry| {
                let _ = futures::executor::block_on(self.tangle_producer_manager.submit_transaction(entry)).unwrap();
            });
        }
    }
}

impl BlockProducerTransactionConsumer {
    pub fn new(spool_client: &Arc<dyn SpoolConnection>, tangle_producer_manager: &Arc<dyn TangleProducerManager>) -> Arc<dyn ProducerTransactionConsumer> {
        let config = ChainConfig::new().unwrap();
        Arc::new(BlockProducerTransactionConsumer { spool_client: spool_client.clone(), tangle_producer_manager: tangle_producer_manager.clone()  }) as Arc<dyn ProducerTransactionConsumer>
    }
}
