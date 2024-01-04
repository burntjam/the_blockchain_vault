use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;
use tonic::transport::Channel;
use tonic::{transport::Server, Request, Response};
use spool::spooler_client::{SpoolerClient};
use spool::{SpoolRequest,SpoolSubscriber,SpoolResponse};
use std::thread;
use std::time::Duration;
use http::Uri;
use std::str::FromStr;
use super::spool_connection;
use std::sync::Arc;

pub mod spool {
    tonic::include_proto!("spool");
}


pub struct SpoolGrpcClient {
    client: SpoolerClient<Channel>,
    topic: String,
}

impl SpoolGrpcClient {
    pub async fn createSession(url: String, topic: String) -> Result<SpoolGrpcClient, Box<dyn std::error::Error>> {
        let uri = Uri::from_str(&url)?;
        let channel = Channel::builder(uri).connect().await?;
        let client = SpoolerClient::new(channel.clone());
        Ok(SpoolGrpcClient {
            client,
            topic,
        })
    }
    pub async fn push(&self,message: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let mut spool_client = self.client.clone();
        let result = spool_client.push(Request::new(SpoolRequest { topic: self.topic.clone(), body: message })).await.unwrap();
        Ok(())
    }
    pub async fn pushToTopic(&self,message: Vec<u8>, topic: &String) -> Result<(), Box<dyn std::error::Error>> {
        let mut spool_client = self.client.clone();
        let result = spool_client.push(Request::new(SpoolRequest { topic: topic.clone(), body: message })).await.unwrap();
        Ok(())
    }
    pub async fn consume(&self) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
        let mut spool_client = self.client.clone();
        let result = spool_client.consume(Request::new(SpoolSubscriber { topic: self.topic.clone(), timeout: 10 })).await.unwrap();
        Ok(result.into_inner().body)
    }
    pub async fn consumeFromTopic(&self, topic: &String) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
        let mut spool_client = self.client.clone();
        let result = spool_client.consume(Request::new(SpoolSubscriber { topic: topic.clone(), timeout: 10 })).await.unwrap();
        Ok(result.into_inner().body)
    }
}


pub struct SpoolGrpcConnection {
    client: SpoolGrpcClient,
}

#[async_trait]
impl spool_connection::SpoolConnection for SpoolGrpcConnection {
    async fn push(&self,message: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        // Then you can use `block_on` to run your async code
        self.client.push(message).await
    }

    async fn pushToTopic(&self,message: Vec<u8>, topic: &String) -> Result<(), Box<dyn std::error::Error>> {
        // Then you can use `block_on` to run your async code
        self.client.pushToTopic(message, topic).await
    }

    async fn consume(&self) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
        // Then you can use `block_on` to run your async code
        self.client.consume().await
    }

    async fn consumeFromTopic(&self, topic: &String) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
        // Then you can use `block_on` to run your async code
        self.client.consumeFromTopic(topic).await
    }

}

pub async fn createGrpcConnection(url: String, topic: String) -> Result<Arc<dyn spool_connection::SpoolConnection>, Box<dyn std::error::Error>> {
    let client = SpoolGrpcClient::createSession(url,topic).await?;

    Ok(Arc::new(SpoolGrpcConnection {
        client,
    }))
}