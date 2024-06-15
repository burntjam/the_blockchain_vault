//! Main library entry point for openapi_client implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios", target_os = "linux")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use rest_stub::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        rest_stub::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use rest_stub::{
    Api,
    AdminUsersGetResponse,
    AdminUsersIdDeleteResponse,
    AdminUsersIdPutResponse,
    DiagramsIdExportPostResponse,
    DiagramsImportPostResponse,
    EntitiesGetResponse,
    EntitiesIdDeleteResponse,
    EntitiesIdGetResponse,
    EntitiesIdPutResponse,
    EntitiesPostResponse,
    GraphsGetResponse,
    GraphsIdDeleteResponse,
    GraphsIdGetResponse,
    GraphsIdPutResponse,
    GraphsPostResponse,
    RelationshipsGetResponse,
    RelationshipsIdDeleteResponse,
    RelationshipsIdGetResponse,
    RelationshipsIdPutResponse,
    RelationshipsPostResponse,
    SparqlQueryPostResponse,
};
use rest_stub::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Get a list of users (for admin users).
    async fn admin_users_get(
        &self,
        context: &C) -> Result<AdminUsersGetResponse, ApiError>
    {
        info!("admin_users_get() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Delete a user (for admin users).
    async fn admin_users_id_delete(
        &self,
        id: String,
        context: &C) -> Result<AdminUsersIdDeleteResponse, ApiError>
    {
        info!("admin_users_id_delete(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Update user roles or permissions (for admin users).
    async fn admin_users_id_put(
        &self,
        id: String,
        context: &C) -> Result<AdminUsersIdPutResponse, ApiError>
    {
        info!("admin_users_id_put(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Export a diagram to a desired format.
    async fn diagrams_id_export_post(
        &self,
        id: String,
        context: &C) -> Result<DiagramsIdExportPostResponse, ApiError>
    {
        info!("diagrams_id_export_post(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Import a diagram from an RDF source.
    async fn diagrams_import_post(
        &self,
        context: &C) -> Result<DiagramsImportPostResponse, ApiError>
    {
        info!("diagrams_import_post() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List all entities in a specific diagram.
    async fn entities_get(
        &self,
        context: &C) -> Result<EntitiesGetResponse, ApiError>
    {
        info!("entities_get() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Remove an entity from a diagram.
    async fn entities_id_delete(
        &self,
        id: String,
        context: &C) -> Result<EntitiesIdDeleteResponse, ApiError>
    {
        info!("entities_id_delete(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Retrieve a specific entity.
    async fn entities_id_get(
        &self,
        id: String,
        context: &C) -> Result<EntitiesIdGetResponse, ApiError>
    {
        info!("entities_id_get(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Update an existing entity.
    async fn entities_id_put(
        &self,
        id: String,
        context: &C) -> Result<EntitiesIdPutResponse, ApiError>
    {
        info!("entities_id_put(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Add a new entity to a diagram.
    async fn entities_post(
        &self,
        context: &C) -> Result<EntitiesPostResponse, ApiError>
    {
        info!("entities_post() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List all graphs.
    async fn graphs_get(
        &self,
        context: &C) -> Result<GraphsGetResponse, ApiError>
    {
        info!("graphs_get() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Delete a graph.
    async fn graphs_id_delete(
        &self,
        id: String,
        context: &C) -> Result<GraphsIdDeleteResponse, ApiError>
    {
        info!("graphs_id_delete(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Retrieve a specific graph.
    async fn graphs_id_get(
        &self,
        id: String,
        context: &C) -> Result<GraphsIdGetResponse, ApiError>
    {
        info!("graphs_id_get(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Update an existing graph.
    async fn graphs_id_put(
        &self,
        id: String,
        context: &C) -> Result<GraphsIdPutResponse, ApiError>
    {
        info!("graphs_id_put(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Create a new graph.
    async fn graphs_post(
        &self,
        context: &C) -> Result<GraphsPostResponse, ApiError>
    {
        info!("graphs_post() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List all relationships in a diagram.
    async fn relationships_get(
        &self,
        context: &C) -> Result<RelationshipsGetResponse, ApiError>
    {
        info!("relationships_get() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Delete a relationship.
    async fn relationships_id_delete(
        &self,
        id: String,
        context: &C) -> Result<RelationshipsIdDeleteResponse, ApiError>
    {
        info!("relationships_id_delete(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Retrieve a specific relationship.
    async fn relationships_id_get(
        &self,
        id: String,
        context: &C) -> Result<RelationshipsIdGetResponse, ApiError>
    {
        info!("relationships_id_get(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Update an existing relationship.
    async fn relationships_id_put(
        &self,
        id: String,
        context: &C) -> Result<RelationshipsIdPutResponse, ApiError>
    {
        info!("relationships_id_put(\"{}\") - X-Span-ID: {:?}", id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Create a new relationship between two entities.
    async fn relationships_post(
        &self,
        context: &C) -> Result<RelationshipsPostResponse, ApiError>
    {
        info!("relationships_post() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Execute a SPARQL query against the underlying RDF data store and return the results.
    async fn sparql_query_post(
        &self,
        context: &C) -> Result<SparqlQueryPostResponse, ApiError>
    {
        info!("sparql_query_post() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

}
