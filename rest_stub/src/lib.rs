#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "1.0.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AdminUsersGetResponse {
    /// List of users retrieved successfully.
    ListOfUsersRetrievedSuccessfully
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AdminUsersIdDeleteResponse {
    /// User deleted successfully.
    UserDeletedSuccessfully
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AdminUsersIdPutResponse {
    /// User updated successfully.
    UserUpdatedSuccessfully
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DiagramsIdExportPostResponse {
    /// Diagram exported successfully.
    DiagramExportedSuccessfully
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DiagramsImportPostResponse {
    /// Diagram imported successfully.
    DiagramImportedSuccessfully
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EntitiesGetResponse {
    /// A list of entities.
    AListOfEntities
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EntitiesIdDeleteResponse {
    /// Entity removed successfully.
    EntityRemovedSuccessfully
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EntitiesIdGetResponse {
    /// Entity details.
    EntityDetails
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EntitiesIdPutResponse {
    /// Entity updated successfully.
    EntityUpdatedSuccessfully
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EntitiesPostResponse {
    /// Entity added successfully.
    EntityAddedSuccessfully
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GraphsGetResponse {
    /// A list of graphs.
    AListOfGraphs
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GraphsIdDeleteResponse {
    /// Graph deleted successfully.
    GraphDeletedSuccessfully
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GraphsIdGetResponse {
    /// Graph details.
    GraphDetails
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GraphsIdPutResponse {
    /// Graph updated successfully.
    GraphUpdatedSuccessfully
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GraphsPostResponse {
    /// Graph created successfully.
    GraphCreatedSuccessfully
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RelationshipsGetResponse {
    /// A list of relationships.
    AListOfRelationships
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RelationshipsIdDeleteResponse {
    /// Relationship deleted successfully.
    RelationshipDeletedSuccessfully
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RelationshipsIdGetResponse {
    /// Relationship details.
    RelationshipDetails
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RelationshipsIdPutResponse {
    /// Relationship updated successfully.
    RelationshipUpdatedSuccessfully
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RelationshipsPostResponse {
    /// Relationship created successfully.
    RelationshipCreatedSuccessfully
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SparqlQueryPostResponse {
    /// Query executed successfully and results returned.
    QueryExecutedSuccessfullyAndResultsReturned
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Get a list of users (for admin users).
    async fn admin_users_get(
        &self,
        context: &C) -> Result<AdminUsersGetResponse, ApiError>;

    /// Delete a user (for admin users).
    async fn admin_users_id_delete(
        &self,
        id: String,
        context: &C) -> Result<AdminUsersIdDeleteResponse, ApiError>;

    /// Update user roles or permissions (for admin users).
    async fn admin_users_id_put(
        &self,
        id: String,
        context: &C) -> Result<AdminUsersIdPutResponse, ApiError>;

    /// Export a diagram to a desired format.
    async fn diagrams_id_export_post(
        &self,
        id: String,
        context: &C) -> Result<DiagramsIdExportPostResponse, ApiError>;

    /// Import a diagram from an RDF source.
    async fn diagrams_import_post(
        &self,
        context: &C) -> Result<DiagramsImportPostResponse, ApiError>;

    /// List all entities in a specific diagram.
    async fn entities_get(
        &self,
        context: &C) -> Result<EntitiesGetResponse, ApiError>;

    /// Remove an entity from a diagram.
    async fn entities_id_delete(
        &self,
        id: String,
        context: &C) -> Result<EntitiesIdDeleteResponse, ApiError>;

    /// Retrieve a specific entity.
    async fn entities_id_get(
        &self,
        id: String,
        context: &C) -> Result<EntitiesIdGetResponse, ApiError>;

    /// Update an existing entity.
    async fn entities_id_put(
        &self,
        id: String,
        context: &C) -> Result<EntitiesIdPutResponse, ApiError>;

    /// Add a new entity to a diagram.
    async fn entities_post(
        &self,
        context: &C) -> Result<EntitiesPostResponse, ApiError>;

    /// List all graphs.
    async fn graphs_get(
        &self,
        context: &C) -> Result<GraphsGetResponse, ApiError>;

    /// Delete a graph.
    async fn graphs_id_delete(
        &self,
        id: String,
        context: &C) -> Result<GraphsIdDeleteResponse, ApiError>;

    /// Retrieve a specific graph.
    async fn graphs_id_get(
        &self,
        id: String,
        context: &C) -> Result<GraphsIdGetResponse, ApiError>;

    /// Update an existing graph.
    async fn graphs_id_put(
        &self,
        id: String,
        context: &C) -> Result<GraphsIdPutResponse, ApiError>;

    /// Create a new graph.
    async fn graphs_post(
        &self,
        context: &C) -> Result<GraphsPostResponse, ApiError>;

    /// List all relationships in a diagram.
    async fn relationships_get(
        &self,
        context: &C) -> Result<RelationshipsGetResponse, ApiError>;

    /// Delete a relationship.
    async fn relationships_id_delete(
        &self,
        id: String,
        context: &C) -> Result<RelationshipsIdDeleteResponse, ApiError>;

    /// Retrieve a specific relationship.
    async fn relationships_id_get(
        &self,
        id: String,
        context: &C) -> Result<RelationshipsIdGetResponse, ApiError>;

    /// Update an existing relationship.
    async fn relationships_id_put(
        &self,
        id: String,
        context: &C) -> Result<RelationshipsIdPutResponse, ApiError>;

    /// Create a new relationship between two entities.
    async fn relationships_post(
        &self,
        context: &C) -> Result<RelationshipsPostResponse, ApiError>;

    /// Execute a SPARQL query against the underlying RDF data store and return the results.
    async fn sparql_query_post(
        &self,
        context: &C) -> Result<SparqlQueryPostResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Get a list of users (for admin users).
    async fn admin_users_get(
        &self,
        ) -> Result<AdminUsersGetResponse, ApiError>;

    /// Delete a user (for admin users).
    async fn admin_users_id_delete(
        &self,
        id: String,
        ) -> Result<AdminUsersIdDeleteResponse, ApiError>;

    /// Update user roles or permissions (for admin users).
    async fn admin_users_id_put(
        &self,
        id: String,
        ) -> Result<AdminUsersIdPutResponse, ApiError>;

    /// Export a diagram to a desired format.
    async fn diagrams_id_export_post(
        &self,
        id: String,
        ) -> Result<DiagramsIdExportPostResponse, ApiError>;

    /// Import a diagram from an RDF source.
    async fn diagrams_import_post(
        &self,
        ) -> Result<DiagramsImportPostResponse, ApiError>;

    /// List all entities in a specific diagram.
    async fn entities_get(
        &self,
        ) -> Result<EntitiesGetResponse, ApiError>;

    /// Remove an entity from a diagram.
    async fn entities_id_delete(
        &self,
        id: String,
        ) -> Result<EntitiesIdDeleteResponse, ApiError>;

    /// Retrieve a specific entity.
    async fn entities_id_get(
        &self,
        id: String,
        ) -> Result<EntitiesIdGetResponse, ApiError>;

    /// Update an existing entity.
    async fn entities_id_put(
        &self,
        id: String,
        ) -> Result<EntitiesIdPutResponse, ApiError>;

    /// Add a new entity to a diagram.
    async fn entities_post(
        &self,
        ) -> Result<EntitiesPostResponse, ApiError>;

    /// List all graphs.
    async fn graphs_get(
        &self,
        ) -> Result<GraphsGetResponse, ApiError>;

    /// Delete a graph.
    async fn graphs_id_delete(
        &self,
        id: String,
        ) -> Result<GraphsIdDeleteResponse, ApiError>;

    /// Retrieve a specific graph.
    async fn graphs_id_get(
        &self,
        id: String,
        ) -> Result<GraphsIdGetResponse, ApiError>;

    /// Update an existing graph.
    async fn graphs_id_put(
        &self,
        id: String,
        ) -> Result<GraphsIdPutResponse, ApiError>;

    /// Create a new graph.
    async fn graphs_post(
        &self,
        ) -> Result<GraphsPostResponse, ApiError>;

    /// List all relationships in a diagram.
    async fn relationships_get(
        &self,
        ) -> Result<RelationshipsGetResponse, ApiError>;

    /// Delete a relationship.
    async fn relationships_id_delete(
        &self,
        id: String,
        ) -> Result<RelationshipsIdDeleteResponse, ApiError>;

    /// Retrieve a specific relationship.
    async fn relationships_id_get(
        &self,
        id: String,
        ) -> Result<RelationshipsIdGetResponse, ApiError>;

    /// Update an existing relationship.
    async fn relationships_id_put(
        &self,
        id: String,
        ) -> Result<RelationshipsIdPutResponse, ApiError>;

    /// Create a new relationship between two entities.
    async fn relationships_post(
        &self,
        ) -> Result<RelationshipsPostResponse, ApiError>;

    /// Execute a SPARQL query against the underlying RDF data store and return the results.
    async fn sparql_query_post(
        &self,
        ) -> Result<SparqlQueryPostResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Get a list of users (for admin users).
    async fn admin_users_get(
        &self,
        ) -> Result<AdminUsersGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().admin_users_get(&context).await
    }

    /// Delete a user (for admin users).
    async fn admin_users_id_delete(
        &self,
        id: String,
        ) -> Result<AdminUsersIdDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().admin_users_id_delete(id, &context).await
    }

    /// Update user roles or permissions (for admin users).
    async fn admin_users_id_put(
        &self,
        id: String,
        ) -> Result<AdminUsersIdPutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().admin_users_id_put(id, &context).await
    }

    /// Export a diagram to a desired format.
    async fn diagrams_id_export_post(
        &self,
        id: String,
        ) -> Result<DiagramsIdExportPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().diagrams_id_export_post(id, &context).await
    }

    /// Import a diagram from an RDF source.
    async fn diagrams_import_post(
        &self,
        ) -> Result<DiagramsImportPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().diagrams_import_post(&context).await
    }

    /// List all entities in a specific diagram.
    async fn entities_get(
        &self,
        ) -> Result<EntitiesGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().entities_get(&context).await
    }

    /// Remove an entity from a diagram.
    async fn entities_id_delete(
        &self,
        id: String,
        ) -> Result<EntitiesIdDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().entities_id_delete(id, &context).await
    }

    /// Retrieve a specific entity.
    async fn entities_id_get(
        &self,
        id: String,
        ) -> Result<EntitiesIdGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().entities_id_get(id, &context).await
    }

    /// Update an existing entity.
    async fn entities_id_put(
        &self,
        id: String,
        ) -> Result<EntitiesIdPutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().entities_id_put(id, &context).await
    }

    /// Add a new entity to a diagram.
    async fn entities_post(
        &self,
        ) -> Result<EntitiesPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().entities_post(&context).await
    }

    /// List all graphs.
    async fn graphs_get(
        &self,
        ) -> Result<GraphsGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().graphs_get(&context).await
    }

    /// Delete a graph.
    async fn graphs_id_delete(
        &self,
        id: String,
        ) -> Result<GraphsIdDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().graphs_id_delete(id, &context).await
    }

    /// Retrieve a specific graph.
    async fn graphs_id_get(
        &self,
        id: String,
        ) -> Result<GraphsIdGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().graphs_id_get(id, &context).await
    }

    /// Update an existing graph.
    async fn graphs_id_put(
        &self,
        id: String,
        ) -> Result<GraphsIdPutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().graphs_id_put(id, &context).await
    }

    /// Create a new graph.
    async fn graphs_post(
        &self,
        ) -> Result<GraphsPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().graphs_post(&context).await
    }

    /// List all relationships in a diagram.
    async fn relationships_get(
        &self,
        ) -> Result<RelationshipsGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().relationships_get(&context).await
    }

    /// Delete a relationship.
    async fn relationships_id_delete(
        &self,
        id: String,
        ) -> Result<RelationshipsIdDeleteResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().relationships_id_delete(id, &context).await
    }

    /// Retrieve a specific relationship.
    async fn relationships_id_get(
        &self,
        id: String,
        ) -> Result<RelationshipsIdGetResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().relationships_id_get(id, &context).await
    }

    /// Update an existing relationship.
    async fn relationships_id_put(
        &self,
        id: String,
        ) -> Result<RelationshipsIdPutResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().relationships_id_put(id, &context).await
    }

    /// Create a new relationship between two entities.
    async fn relationships_post(
        &self,
        ) -> Result<RelationshipsPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().relationships_post(&context).await
    }

    /// Execute a SPARQL query against the underlying RDF data store and return the results.
    async fn sparql_query_post(
        &self,
        ) -> Result<SparqlQueryPostResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().sparql_query_post(&context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
