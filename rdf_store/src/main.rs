mod store_manager;

use store_manager::*;

#[tokio::main]
async fn main() {
    let storeManager = StoreManager::new();
}