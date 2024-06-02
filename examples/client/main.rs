#![allow(missing_docs, unused_variables, trivial_casts)]


#[allow(unused_imports)]
use futures::{future, Stream, stream};
#[allow(unused_imports)]
use openapi_client::{Api, ApiNoContext, Client, ContextWrapperExt, models,
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
use clap::{App, Arg};

#[allow(unused_imports)]
use log::info;

// swagger::Has may be unused if there are no examples
#[allow(unused_imports)]
use swagger::{AuthData, ContextBuilder, EmptyContext, Has, Push, XSpanIdString};

type ClientContext = swagger::make_context_ty!(ContextBuilder, EmptyContext, Option<AuthData>, XSpanIdString);

// rt may be unused if there are no examples
#[allow(unused_mut)]
fn main() {
    env_logger::init();

    let matches = App::new("client")
        .arg(Arg::with_name("operation")
            .help("Sets the operation to run")
            .possible_values(&[
                "AdminUsersGet",
                "AdminUsersIdDelete",
                "AdminUsersIdPut",
                "DiagramsIdExportPost",
                "DiagramsImportPost",
                "EntitiesGet",
                "EntitiesIdDelete",
                "EntitiesIdGet",
                "EntitiesIdPut",
                "EntitiesPost",
                "GraphsGet",
                "GraphsIdDelete",
                "GraphsIdGet",
                "GraphsIdPut",
                "GraphsPost",
                "RelationshipsGet",
                "RelationshipsIdDelete",
                "RelationshipsIdGet",
                "RelationshipsIdPut",
                "RelationshipsPost",
                "SparqlQueryPost",
            ])
            .required(true)
            .index(1))
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .arg(Arg::with_name("host")
            .long("host")
            .takes_value(true)
            .default_value("localhost")
            .help("Hostname to contact"))
        .arg(Arg::with_name("port")
            .long("port")
            .takes_value(true)
            .default_value("8080")
            .help("Port to contact"))
        .get_matches();

    let is_https = matches.is_present("https");
    let base_url = format!("{}://{}:{}",
                           if is_https { "https" } else { "http" },
                           matches.value_of("host").unwrap(),
                           matches.value_of("port").unwrap());

    let context: ClientContext =
        swagger::make_context!(ContextBuilder, EmptyContext, None as Option<AuthData>, XSpanIdString::default());

    let mut client : Box<dyn ApiNoContext<ClientContext>> = if matches.is_present("https") {
        // Using Simple HTTPS
        let client = Box::new(Client::try_new_https(&base_url)
            .expect("Failed to create HTTPS client"));
        Box::new(client.with_context(context))
    } else {
        // Using HTTP
        let client = Box::new(Client::try_new_http(
            &base_url)
            .expect("Failed to create HTTP client"));
        Box::new(client.with_context(context))
    };

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    match matches.value_of("operation") {
        Some("AdminUsersGet") => {
            let result = rt.block_on(client.admin_users_get(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AdminUsersIdDelete") => {
            let result = rt.block_on(client.admin_users_id_delete(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("AdminUsersIdPut") => {
            let result = rt.block_on(client.admin_users_id_put(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DiagramsIdExportPost") => {
            let result = rt.block_on(client.diagrams_id_export_post(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("DiagramsImportPost") => {
            let result = rt.block_on(client.diagrams_import_post(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("EntitiesGet") => {
            let result = rt.block_on(client.entities_get(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("EntitiesIdDelete") => {
            let result = rt.block_on(client.entities_id_delete(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("EntitiesIdGet") => {
            let result = rt.block_on(client.entities_id_get(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("EntitiesIdPut") => {
            let result = rt.block_on(client.entities_id_put(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("EntitiesPost") => {
            let result = rt.block_on(client.entities_post(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GraphsGet") => {
            let result = rt.block_on(client.graphs_get(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GraphsIdDelete") => {
            let result = rt.block_on(client.graphs_id_delete(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GraphsIdGet") => {
            let result = rt.block_on(client.graphs_id_get(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GraphsIdPut") => {
            let result = rt.block_on(client.graphs_id_put(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("GraphsPost") => {
            let result = rt.block_on(client.graphs_post(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("RelationshipsGet") => {
            let result = rt.block_on(client.relationships_get(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("RelationshipsIdDelete") => {
            let result = rt.block_on(client.relationships_id_delete(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("RelationshipsIdGet") => {
            let result = rt.block_on(client.relationships_id_get(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("RelationshipsIdPut") => {
            let result = rt.block_on(client.relationships_id_put(
                  "id_example".to_string()
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("RelationshipsPost") => {
            let result = rt.block_on(client.relationships_post(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        Some("SparqlQueryPost") => {
            let result = rt.block_on(client.sparql_query_post(
            ));
            info!("{:?} (X-Span-ID: {:?})", result, (client.context() as &dyn Has<XSpanIdString>).get().clone());
        },
        _ => {
            panic!("Invalid operation provided")
        }
    }
}
