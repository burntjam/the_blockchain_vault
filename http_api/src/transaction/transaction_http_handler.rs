use std::{collections::HashMap, sync::{Arc,Mutex}};
use actix_web::{get, http, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder, http::header::ContentType};
use async_trait::async_trait;
use crate::{HttpHandler};
use http_api_messages::{HttpApiRequest,HttpApiResponse,HttpApiError};
use rdf_store_client::StoreClientManager;

pub struct TransactionHttpHandler {
    store_client_manager: Arc<Mutex<dyn StoreClientManager>>,
}

impl TransactionHttpHandler {
    pub fn new(store_client_manager: &Arc<Mutex<dyn StoreClientManager>>) -> Result<Arc<dyn HttpHandler>,Box<dyn std::error::Error>> {
        Ok(Arc::new(TransactionHttpHandler{store_client_manager: store_client_manager.clone()}))
    }
    pub fn get_transactions(&self,_req: &HttpApiRequest) -> Result<String,Box<dyn std::error::Error>> {
        let store_client_manager = self.store_client_manager.clone();
        let rdf_store_client = store_client_manager.lock().unwrap();
        let store_client = rdf_store_client.create_client()?;
        let store_client = store_client.lock().unwrap();
        let rdf_query = if let Some(value) = _req.parameters.get("contract_id") {format!(
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
                ORDER BY DESC (?dateTime) LIMIT 1"#,value)} 
                else if let Some(value) = _req.parameters.get("contract_name") {format!(
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
                        ORDER BY DESC (?dateTime) LIMIT 1"#,value)}
                else {
                    return Err(Box::new(HttpApiError{message:String::from("The contract_id or contract_name has not been provided")}));
                };
        let query_result = futures::executor::block_on(store_client.query_async(&rdf_query))?;
        if query_result.rows.is_empty() {
            return Err(Box::new(HttpApiError{message:String::from("The contract was not found")}));
        }
        if let Some(value) = query_result.rows.get(0) {
            if let Some(value) = value.get("code") {
                if let rdf_lib::RdfCellValue::Text(value) = value.value.clone() {
                    return Ok(value)
                } else {
                    return Err(Box::new(HttpApiError{message:String::from("Cannot extract code out of store")}));
                }
            } else {
                return Err(Box::new(HttpApiError{message:String::from("No code returned")}));
            }
        } else {
            return Err(Box::new(HttpApiError{message:String::from("The contract was not found")}));
        }
    }
}

#[async_trait]
impl HttpHandler for TransactionHttpHandler {
    async fn process(&self, _req: HttpApiRequest) -> Result<HttpApiResponse,Box<dyn std::error::Error>> {
        let transaction_json = self.get_transactions(&_req)?;

        Ok(HttpApiResponse{
            body: transaction_json,
            parameters: HashMap::new(),
            content_type: String::from("application/json"),
        })
    }
}