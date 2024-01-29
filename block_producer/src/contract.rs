use std::sync::{Arc,Mutex};
use config_lib::ChainConfig;
use rdf_lib::StoreSessionFactory;


pub trait Contract: Sync + Send {
    fn contract_id(&self) -> Result<Vec<u8>,Box<dyn std::error::Error>>;
    fn contract(&self) -> Result<Vec<u8>,Box<dyn std::error::Error>>;
}


pub struct BlockContract {
    contract_id: Vec<u8>,
    contract: Vec<u8>,
    session_factory: Arc<Mutex<dyn StoreSessionFactory>>,
}


impl BlockContract {
    pub fn new(contract_id: &Vec<u8>, contract: &Vec<u8>, session_factory: Arc<Mutex<dyn StoreSessionFactory>>) -> Result<Arc<Mutex<dyn Contract>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        Ok(Arc::new(Mutex::new(BlockContract { contract_id: contract_id.clone(), contract: contract.clone(), session_factory })) as Arc<Mutex<dyn Contract>>)
    }
}

impl Contract for BlockContract {
    fn contract_id(&self) -> Result<Vec<u8>,Box<dyn std::error::Error>> {
        Ok(self.contract_id.clone())
    }
    fn contract(&self) -> Result<Vec<u8>,Box<dyn std::error::Error>> {
        Ok(self.contract.clone())
    }
}


#[cfg(test)]
mod tests {
    use crate::transaction_manager;

    use super::*;
    use std::error::Error;
    use spool_client::*;
    use spool_errors::*;
    use std::sync::{Mutex, Arc,mpsc};
    use rdf_lib::MockStoreSessionFactory;

    #[test]
    fn test_block_contract_id() -> Result<(), Box<dyn Error>> {
        let contract = BlockContract::new(&vec![1,2,3,4], &vec![1,2,3,4], MockStoreSessionFactory::new()?)?;
        let contract_ref = contract.lock().unwrap();
        assert_eq!(contract_ref.contract_id()?,vec![1,2,3,4]);
        Ok(())
    }

    #[test]
    fn test_block_contract() -> Result<(), Box<dyn Error>> {
        let contract = BlockContract::new(&vec![1,2,3,4], &vec![1,2,3,4], MockStoreSessionFactory::new()?)?;
        let contract_ref = contract.lock().unwrap();
        assert_eq!(contract_ref.contract()?,vec![1,2,3,4]);
        Ok(())
    }
}