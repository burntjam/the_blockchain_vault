use std::sync::Arc;
pub mod block_db_manager;
pub mod transaction_processor;
pub mod transaction_manager;
pub mod transaction_consumer;
pub mod transaction_constants;
pub mod transaction_errors;
pub mod mock;

pub use block_db_manager::*;
pub use transaction_processor::*;
pub use transaction_manager::*;
pub use transaction_consumer::*;
pub use transaction_constants::*;
pub use transaction_errors::*;
pub use mock::*;

#[tokio::main]
async fn main() {
    let block_db_manager = BlockDbManager::new();

}