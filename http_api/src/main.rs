use actix_web::{get, web, http, App, HttpResponse, HttpServer, Responder, HttpRequest};
use config_lib::ChainConfig;

async fn hello_world(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn rdf_query(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Rdf")
}

async fn contract(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Contract")
}

async fn transaction(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Transaction")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = ChainConfig::new().unwrap();
    HttpServer::new(|| {
        App::new()
            .route("/", web::method(http::Method::GET).to(hello_world))
            .route("/rdf/", web::method(http::Method::GET).to(rdf_query))
            .route("/contract/", web::method(http::Method::GET).to(contract))
            .route("/transaction/", web::method(http::Method::GET).to(transaction))
    })
    .bind(config.http.binding)?
    .run()
    .await
}