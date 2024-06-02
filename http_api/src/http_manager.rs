use crate::{HttpHandler,RdfHttpHandler,ContractHttpHandler,TransactionHttpHandler};
use std::sync::{Arc,Mutex};
use http_api_messages::{HttpApiRequest,HttpApiResponse,HttpApiError};
use rdf_store_client::StoreClientManager;
use sandbox_client::RequestManager;

pub trait HttpManager {
    fn get_handler(&self,handler_name: &String) -> Result<Arc<dyn HttpHandler>,Box<dyn std::error::Error>>;
}


pub struct ApiHttpManager {
    store_client_manager: Arc<Mutex<dyn StoreClientManager>>,
    sandbox_client_request_manager: Arc<dyn RequestManager>,
}

impl ApiHttpManager {
    pub fn new(store_client_manager: &Arc<Mutex<dyn StoreClientManager>>, sandbox_client_request_manager: &Arc<dyn RequestManager>) -> Result<Arc<dyn HttpManager>,Box<dyn std::error::Error>> {
        Ok(Arc::new(ApiHttpManager{store_client_manager: store_client_manager.clone(), 
            sandbox_client_request_manager: sandbox_client_request_manager.clone()}))
    }
}

impl HttpManager for ApiHttpManager {
    fn get_handler(&self,handler_name: &String) -> Result<Arc<dyn HttpHandler>,Box<dyn std::error::Error>> {
        if handler_name.eq_ignore_ascii_case("RDF") {
            return Ok(RdfHttpHandler::new(&self.store_client_manager)?);
        } else if handler_name.eq_ignore_ascii_case("CONTRACT") {
            return Ok(ContractHttpHandler::new(&self.store_client_manager,&self.sandbox_client_request_manager)?);
        } else if handler_name.eq_ignore_ascii_case("TRANSACTION") {
            return Ok(TransactionHttpHandler::new(&self.store_client_manager)?);
        }
        Err(Box::new(HttpApiError{message:String::from("")}))
    }
}