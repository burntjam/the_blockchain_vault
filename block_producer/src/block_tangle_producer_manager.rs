use config_lib::ChainConfig;
use std::{os::unix::thread, sync::{Arc,Mutex}};
use async_trait::async_trait;
use std::time::Duration;
use crate::{ProducerClient,BlockProducerClient,TangleProducer,BlockTangleProducer, DbManager};
use peer2peer_protocol::{deserialize_bin_message, WebSocketMessage, handler::message, TransactionMessage};
use spool_client::SpoolConnectionManager;
use rdf_store_client::StoreClientManager;

#[async_trait]
pub trait TangleProducerManager: Sync + Send {
    async fn submit_transaction(&self,transaction: &Vec<u8>) -> Result<(),Box<dyn std::error::Error>>;
    async fn init_tangle_producer(&self,id: &Vec<u8>) -> Result<(),Box<dyn std::error::Error>>;
    async fn block_processor(&self) -> Result<(),Box<dyn std::error::Error>>;
}

pub struct BlockTangleProducerManagerStatus {
    active: bool,
}

impl BlockTangleProducerManagerStatus {
    fn new() -> Arc<Mutex<BlockTangleProducerManagerStatus>> {
        Arc::new(Mutex::new(BlockTangleProducerManagerStatus{active:true}))
    }
    fn deactivate(&mut self) -> Result<(),Box<dyn std::error::Error>> {
        self.active = false;
        Ok(())
    }
    fn is_active(&self) -> Result<bool,Box<dyn std::error::Error>> {
        Ok(self.active)
    }
}


pub struct BlockTangleProducerManager {
    producers: Arc<Mutex<Vec<Arc<dyn TangleProducer>>>>,
    status: Arc<Mutex<BlockTangleProducerManagerStatus>>,
    block_db_manager: Arc<dyn DbManager>,
    store_client_manager: Arc<Mutex<dyn StoreClientManager>>
}

impl BlockTangleProducerManager {
    pub fn new(block_db_manager: &Arc<dyn DbManager>, store_client_manager: &Arc<Mutex<dyn StoreClientManager>>) -> Arc<dyn TangleProducerManager> {
        let config = ChainConfig::new().unwrap();
        Arc::new(BlockTangleProducerManager {producers: Arc::new(Mutex::new(Vec::new())), 
            status: BlockTangleProducerManagerStatus::new(), 
            block_db_manager: block_db_manager.clone(),
            store_client_manager: store_client_manager.clone()}) as Arc<dyn TangleProducerManager>
    }
    fn add_transaction(&self,transaction_message: &TransactionMessage) -> Result<(),Box<dyn std::error::Error>> {
        let mut producers = self.producers.lock().unwrap();
        if producers.is_empty() {
            return Ok(())
        };
        if let Some(valid_entry) = producers.iter().find(|&entry| 
            futures::executor::block_on(entry.is_valid_transaction(transaction_message))) {
                futures::executor::block_on(valid_entry.submit_transaction(transaction_message))?;
        };
        Ok(())
    }
    fn check_status(&self) -> bool {
        let status = self.status.lock().unwrap();
        status.is_active().unwrap()
    }
}

#[async_trait]
impl TangleProducerManager for BlockTangleProducerManager {
    async fn submit_transaction(&self,transaction: &Vec<u8>) -> Result<(),Box<dyn std::error::Error>> {
        println!("From process1");
        let message = deserialize_bin_message(&transaction);
        println!("From process2");
        if message.is_err() {
            return Err(message.err().unwrap());
        }
        println!("From processing");
        if let WebSocketMessage::Transaction(trans) = message.unwrap() {
            println!("More processing");
            self.add_transaction(&trans)?;
        }
        Ok(())
    }
    async fn init_tangle_producer(&self,id: &Vec<u8>) -> Result<(),Box<dyn std::error::Error>> {
        futures::executor::block_on(async move {
            let mut producers = self.producers.lock().unwrap();
            let found_entry = producers.iter().find(|&entry| 
                futures::executor::block_on(entry.id()).unwrap() == id.clone());
            if let None = found_entry {
                producers.push(BlockTangleProducer::new(&id,&self.block_db_manager,&self.store_client_manager).await?);
            };
            Ok(())
        })
    }
    async fn block_processor(&self) -> Result<(),Box<dyn std::error::Error>> {
        while self.check_status() {
            let mut producers = self.producers.lock().unwrap();
            producers.iter().for_each(|producer| {
                futures::executor::block_on(producer.process()).unwrap();
            });
            // create a block every 10 seconds
            std::thread::sleep(Duration::from_millis(10*1000));
        }
        Ok(())
    }
}
