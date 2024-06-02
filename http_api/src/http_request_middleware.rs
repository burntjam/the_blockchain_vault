use actix_service::{Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse, Payload},Error, HttpMessage, HttpResponse, http::header};
use actix_web::body::{EitherBody, MessageBody, BoxBody}; // Corrected import paths
use futures::future::{ok, Ready, self, BoxFuture};
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::{http_manager, ApiHttpManager, HttpApiRequest, HttpManager};
use std::collections::HashMap;
use std::sync::{Arc,Mutex};

use futures::StreamExt;
use bytes::BytesMut;


async fn extract_raw_body(mut payload: Payload) -> Result<Vec<u8>, Error> {
    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
    }
    Ok(body.to_vec())
}

async fn translate_request(mut _req: ServiceRequest) -> HttpApiRequest {
    let payload = _req.take_payload();
    let body = extract_raw_body(payload).await.unwrap();
    
    HttpApiRequest{
        body: String::from_utf8(body).unwrap(),
        parameters: HashMap::new(),
        url: _req.uri().to_string(),
    }
}

async fn handle_request(mut req: ServiceRequest, http_manager: Arc<dyn HttpManager>, handler_name: &String) -> ServiceResponse { 
    let handler = http_manager.get_handler(&handler_name).unwrap();
    let http_request = req.parts().0.clone();
    let request = translate_request(req).await;
    let handler_result = handler.process(request).await;
    let response = if handler_result.is_err() {
        let error = handler_result.unwrap_err();
        HttpResponse::BadRequest().body(error.to_string())
    } else {
        let result = handler_result.unwrap();
        HttpResponse::Ok().content_type(result.content_type)
            .body(result.body)
    };
    
    //let result = handler.unwrap().process(request).await;
    ServiceResponse::new(http_request, response)
}


// Define your middleware struct. It can contain configurations or shared resources.
pub struct RequestMiddleware {
    pub http_manager: Arc<dyn HttpManager>,
    
}

// Implement Transform trait for your middleware. This trait initializes your middleware's state.
impl<S, B> Transform<S, ServiceRequest> for RequestMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B:  MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = RequestMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestMiddlewareService { service, http_manager: self.http_manager.clone() })
    }
}

// Define the service produced by the middleware.
pub struct RequestMiddlewareService<S> {
    service: S,
    http_manager: Arc<dyn HttpManager>,
}

// Implement the service trait for the middleware service. This is where you handle each request.
impl<S, B> Service<ServiceRequest> for RequestMiddlewareService<S>
where
S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
S::Future: 'static,
B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        println!("Request: {} {}", req.method(), req.path());
        let http_manager = self.http_manager.clone();
        
        // Here you can implement logic based on the request to dynamically load modules, etc.
        if req.path().starts_with("/test") {
            println!("This is a test");

            return Box::pin(async { 
                let response = HttpResponse::Ok()
                    .body("This is a special response for /test")
                        .map_into_boxed_body();
                let res = ServiceResponse::new(req.into_parts().0, response);    
                Ok(res) });

        } else if req.path().starts_with("/contract") {
            println!("This is contract");
            return Box::pin(async { 
                Ok(handle_request(req,http_manager, &String::from("CONTRACT")).await)
            });
        } else if req.path().starts_with("/rdf") {
            println!("This is contract");
            return Box::pin(async { 
                Ok(handle_request(req,http_manager, &String::from("RDF")).await)
            });
        } else if req.path().starts_with("/transaction") {
            println!("This is contract");
            return Box::pin(async { 
                Ok(handle_request(req,http_manager, &String::from("TRANSACTION")).await)
            });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res: ServiceResponse<B> = fut.await?;
            println!("after : ");
            // You can also modify the response here if needed
            // You can also modify the response here if needed
            let res: ServiceResponse<BoxBody> = res.map_into_boxed_body();
            Ok(res)
        })
    }
}
