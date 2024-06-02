use std::{collections::HashMap, sync::{Arc,Mutex}};
use actix_web::{get, http, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, http::header::ContentType};
use async_trait::async_trait;
use crate::{HttpHandler};
use http_api_messages::{HttpApiRequest,HttpApiResponse,HttpApiError};
use rdf_store_client::StoreClientManager;
use serde_json::{Value, json};

pub struct RdfHttpHandler {
    store_client_manager: Arc<Mutex<dyn StoreClientManager>>,
    
}

impl RdfHttpHandler {
    pub fn new(store_client_manager: &Arc<Mutex<dyn StoreClientManager>>) -> Result<Arc<dyn HttpHandler>,Box<dyn std::error::Error>> {
        Ok(Arc::new(RdfHttpHandler{store_client_manager: store_client_manager.clone()}))
    }
    pub fn execute_query(&self,_req: &HttpApiRequest) -> Result<String,Box<dyn std::error::Error>> {
        let store_client_manager = self.store_client_manager.clone();
        let rdf_store_client = store_client_manager.lock().unwrap();
        let store_client = rdf_store_client.create_client()?;
        let store_client = store_client.lock().unwrap();
        let rdf_query = if let Some(value) = _req.parameters.get("query") {
            value 
        } else {
            return Err(Box::new(HttpApiError{message:String::from("The contract_id or contract_name has not been provided")}));
        };
        let query_result = futures::executor::block_on(store_client.query_async(&rdf_query))?;
        if query_result.rows.is_empty() {
            return Err(Box::new(HttpApiError{message:String::from("The query failed")}));
        }

        return Ok(serde_json::to_string(&query_result.rows)?);
    }
}

#[async_trait]
impl HttpHandler for RdfHttpHandler {
    async fn process(&self, _req: HttpApiRequest) -> Result<HttpApiResponse,Box<dyn std::error::Error>> {
        let result_json = self.execute_query(&_req)?;

        Ok(HttpApiResponse{
            body: result_json,
            parameters: HashMap::new(),
            content_type: String::from("application/json"),
        })
    }
}