pub mod http_handler;
pub mod http_manager;
pub mod rdf;
pub mod contract;
pub mod transaction;
pub mod http_logging;
pub mod http_request_middleware;

use actix_web::{get, http, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder};
use config_lib::ChainConfig;
use http_api_messages::HttpApiRequest;
use http_handler::*;
use http_manager::*;
use http_logging::*;
use http_request_middleware::RequestMiddleware;
use rdf::*;
use contract::*;
use rdf_store_client::SpoolStoreClientManager;
use spool_client::SpoolConnectionManagerGrpc;
use sandbox_client::{RequestManager,SpoolRequestManager};
use transaction::*;
use std::collections::HashMap;

async fn hello_world(_req: HttpRequest) -> impl Responder {
    println!("base {}",_req.uri().to_string());
    HttpResponse::Ok().body("Hello, world!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = ChainConfig::new().unwrap();
    let spool_manager = SpoolConnectionManagerGrpc::new(config.spool.url.clone()).await.unwrap();
    let store_client_manager = SpoolStoreClientManager::new(
        &String::from("HTTP_API_RDF"), &spool_manager).unwrap();
    let sandbox_client_request_manager = SpoolRequestManager::new(
        &String::from("HTTP_API_SANDBOX"), &spool_manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(RequestMiddleware{http_manager: ApiHttpManager::new(&store_client_manager,&sandbox_client_request_manager).unwrap()})
            .wrap(LoggingMiddleware)
            .route("/", web::method(http::Method::GET).to(hello_world))
            //.route("/rdf/", web::method(http::Method::GET).to(rdf_query))
            //.route("/ontology/", web::method(http::Method::GET).to(rdf_query))
            //.route("/contract/", web::method(http::Method::GET).to(contract))
            //.route("/transaction/", web::method(http::Method::GET).to(transaction))
    })
    .bind(config.http.binding)?
    .run()
    .await
}