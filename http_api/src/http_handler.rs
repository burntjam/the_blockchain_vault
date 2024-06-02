use actix_web::{get, http, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder};
use config_lib::ChainConfig;
use async_trait::async_trait;
use http_api_messages::{HttpApiRequest,HttpApiResponse};


#[async_trait]
pub trait HttpHandler: Send + Sync {
    async fn process(&self,_req: HttpApiRequest) -> Result<HttpApiResponse,Box<dyn std::error::Error>>;
}