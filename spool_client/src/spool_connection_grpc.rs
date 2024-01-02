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
    rt: tokio::runtime::Runtime,
    client: SpoolGrpcClient,
}

impl spool_connection::SpoolConnection for SpoolGrpcConnection {
    fn push(&self,message: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        // Then you can use `block_on` to run your async code
        self.rt.block_on(async {
            self.client.push(message).await
        })
    }
    fn pushToTopic(&self,message: Vec<u8>, topic: &String) -> Result<(), Box<dyn std::error::Error>> {
        // Then you can use `block_on` to run your async code
        self.rt.block_on(async {
            self.client.pushToTopic(message, topic).await
        })
    }
    fn consume(&self) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
        // Then you can use `block_on` to run your async code
        self.rt.block_on(async {
            self.client.consume().await
        })
    }
    fn consumeFromTopic(&self, topic: &String) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
        // Then you can use `block_on` to run your async code
        self.rt.block_on(async {
            self.client.consumeFromTopic(topic).await
        })
    }

}

pub fn createGrpcConnection(url: String, topic: String) -> Result<Arc<dyn spool_connection::SpoolConnection>, Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let client = rt.block_on(async {
        SpoolGrpcClient::createSession(url,topic).await.unwrap()
    });

    Ok(Arc::new(SpoolGrpcConnection {
        rt,
        client 
    }))
}