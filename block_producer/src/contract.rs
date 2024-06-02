use async_trait::async_trait;
use std::sync::{Arc,Mutex};
use config_lib::ChainConfig;
use rdf_lib::StoreSessionFactory;
use rdf_lib::store_result_set::*;
use rdf_store_client::{StoreClientManager,StoreClient};

use crate::contract_errors;


pub trait Contract: Sync + Send {
    fn contract_id(&self) -> Result<Vec<u8>,Box<dyn std::error::Error>>;
    fn contract(&self) -> Result<Vec<u8>,Box<dyn std::error::Error>>;
}


pub struct BlockContract {
    contract_id: Vec<u8>,
    contract: Vec<u8>,
    store_client_manager: Arc<Mutex<dyn StoreClientManager>>,
}


impl BlockContract {
    pub fn new(contract_id: &Vec<u8>, contract_name: &String,store_client_manager: &Arc<Mutex<dyn StoreClientManager>>) -> Result<Arc<Mutex<dyn Contract>>,Box<dyn std::error::Error>> {
        let config = ChainConfig::new()?;
        let store_client_manager_ref = store_client_manager.lock().unwrap();
        let store_client = store_client_manager_ref.create_client()?;
        let store_client_ref = store_client.lock().unwrap();
        let rdf_query = if contract_id.len() > 0 {format!(
            r#"SELECT ?code ?accountHash ?contractName ?contractNamespace ?contractHash  WHERE {{ 
                ?contract <http://keto-coin.io/schema/rdf/1.0/keto/Contract#hash> '{}'^^<http://www.w3.org/2001/XMLSchema#string> .
                FILTER (STRSTARTS(STR(?contract),'http://keto-coin.io/schema/rdf/1.0/keto/Contract'))
                ?contract <http://keto-coin.io/schema/rdf/1.0/keto/Contract#accountHash> ?accountHash .
                ?contract <http://keto-coin.io/schema/rdf/1.0/keto/Contract#name> ?contractName .
                ?contract <http://keto-coin.io/schema/rdf/1.0/keto/Contract#hash> ?contractHash .
                ?contract <http://keto-coin.io/schema/rdf/1.0/keto/Contract#namespace> ?contractNamespace .
                ?contractVersion <http://keto-coin.io/schema/rdf/1.0/keto/ContractVersion#contract> ?contract .
                ?contractVersion <http://keto-coin.io/schema/rdf/1.0/keto/ContractVersion#dateTime> ?dateTime .
                ?contractVersion <http://keto-coin.io/schema/rdf/1.0/keto/ContractVersion#value> ?code . }}
                ORDER BY DESC (?dateTime) LIMIT 1"#,String::from_utf8(contract_id.clone()).unwrap())} 
                else {format!(
                    r#"SELECT ?code ?accountHash ?contractName ?contractNamespace ?contractHash  WHERE {{ 
                        ?contract <http://keto-coin.io/schema/rdf/1.0/keto/Contract#name> '{}'^^<http://www.w3.org/2001/XMLSchema#string> .
                        FILTER (STRSTARTS(STR(?contract),'http://keto-coin.io/schema/rdf/1.0/keto/Contract'))
                        ?contract <http://keto-coin.io/schema/rdf/1.0/keto/Contract#accountHash> ?accountHash .
                        ?contract <http://keto-coin.io/schema/rdf/1.0/keto/Contract#name> ?contractName .
                        ?contract <http://keto-coin.io/schema/rdf/1.0/keto/Contract#hash> ?contractHash .
                        ?contract <http://keto-coin.io/schema/rdf/1.0/keto/Contract#namespace> ?contractNamespace .
                        ?contractVersion <http://keto-coin.io/schema/rdf/1.0/keto/ContractVersion#contract> ?contract .
                        ?contractVersion <http://keto-coin.io/schema/rdf/1.0/keto/ContractVersion#dateTime> ?dateTime .
                        ?contractVersion <http://keto-coin.io/schema/rdf/1.0/keto/ContractVersion#value> ?code . }}
                        ORDER BY DESC (?dateTime) LIMIT 1"#,contract_name.clone())};

        let result = 
            futures::executor::block_on(async {store_client_ref.query_async(&rdf_query).await})?;
        if result.rows.len() == 0 {
            return Err(Box::new(contract_errors::ContractError { message: "Contract not found".to_string() }))
        }
        let rdf_cell = &result.rows[0]["code"];
        if let rdf_lib::RdfCellValue::Text(value) = rdf_cell.value.clone() {
            return Ok(Arc::new(Mutex::new(BlockContract { contract_id: contract_id.clone(), contract: value.into_bytes(), 
                store_client_manager: store_client_manager.clone()  })) as Arc<Mutex<dyn Contract>>);
        }
        return Err(Box::new(contract_errors::ContractError { message: "Contract not found".to_string() }))
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
    use std::{error::Error, collections::HashMap};
    use spool_client::*;
    use spool_errors::*;
    use std::sync::{Mutex, Arc,mpsc};
    use rdf_lib::MockStoreSessionFactory;
    use rdf_store_client::MockStoreClientManager;

    pub struct ContractMockStoreClient;

    impl ContractMockStoreClient {
        pub fn new() -> Result<Arc<Mutex<dyn StoreClient>>,Box<dyn std::error::Error>> {
            let config = ChainConfig::new()?;
            Ok(Arc::new(Mutex::new(ContractMockStoreClient {})) as Arc<Mutex<dyn StoreClient>>)
        }
    }

    #[async_trait]
    impl StoreClient for ContractMockStoreClient {
        async fn query_async(&self, query: &String) -> Result<RdfResultSet, Box<dyn std::error::Error>> {
            let mut column_headings = Vec::new();
            let mut row: HashMap<String, RdfCell> = HashMap::new();
            let heading1 = String::from("code");
            row.insert(heading1.clone(), RdfCell{name:heading1.clone(), value:RdfCellValue::Text(String::from("Test code"))});
            column_headings.push(heading1);
            let heading2 = String::from("accountHash");
            row.insert(heading2.clone(), RdfCell{name:heading2.clone(), value:RdfCellValue::Text(String::from("accountHash"))});
            column_headings.push(heading2);
            let heading3 = String::from("contractName");
            row.insert(heading3.clone(), RdfCell{name:heading3.clone(), value:RdfCellValue::Text(String::from("contractName"))});
            column_headings.push(heading3);
            let heading4 = String::from("contractNamespace");
            row.insert(heading4.clone(), RdfCell{name:heading4.clone(), value:RdfCellValue::Text(String::from("contractNamespace"))});
            column_headings.push(heading4);
            let heading5 = String::from("contractHash");
            row.insert(heading5.clone(), RdfCell{name:heading5.clone(), value:RdfCellValue::Text(String::from("contractHash"))});
            column_headings.push(heading5);
            let mut rows: Vec<HashMap<String,RdfCell>> = Vec::new();
            rows.push(row);

            Ok(RdfResultSet{column_headings,rows:rows})
        }
        async fn persist_signed_block(&self, signed_block: &Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
    }

    pub struct ContractMockStoreClientManager;

    impl ContractMockStoreClientManager {
        pub fn new() -> Result<Arc<Mutex<dyn StoreClientManager>>,Box<dyn std::error::Error>> {
            let config = ChainConfig::new()?;
            Ok(Arc::new(Mutex::new(ContractMockStoreClientManager {})) as Arc<Mutex<dyn StoreClientManager>>)
        }
    }

    impl StoreClientManager for ContractMockStoreClientManager {
        fn create_client(&self) -> Result<Arc<Mutex<dyn StoreClient>>, Box<dyn std::error::Error>> {
            Ok(self.create_client_by_name(&String::from("MOCK"))?)
        }
        fn create_client_by_name(&self, client_name: &String) -> Result<Arc<Mutex<dyn StoreClient>>, Box<dyn std::error::Error>> {
            Ok(ContractMockStoreClient::new()?)
        }
    }

    #[tokio::test]
    async fn test_block_contract_id() -> Result<(), Box<dyn Error>> {
        let contract = 
            BlockContract::new(&vec![1,2,3,4], &String::from(""),&ContractMockStoreClientManager::new()?)?;
        let contract_ref = contract.lock().unwrap();
        assert_eq!(contract_ref.contract_id()?,vec![1,2,3,4]);
        Ok(())
    }

    #[tokio::test]
    async fn test_block_contract_name() -> Result<(), Box<dyn Error>> {
        let contract = 
            BlockContract::new(&vec![], &String::from("TEST"), 
                &ContractMockStoreClientManager::new()?)?;
        let contract_ref = contract.lock().unwrap();
        assert_eq!(contract_ref.contract()?,String::from("Test code").as_bytes());
        Ok(())
    }
}