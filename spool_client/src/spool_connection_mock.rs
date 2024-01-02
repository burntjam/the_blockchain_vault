use tonic::transport::Channel;
use tonic::{transport::Server, Request, Response};
use super::spool::spooler_client::{SpoolerClient};
use super::spool::{SpoolRequest,SpoolSubscriber,SpoolResponse};
use std::thread;
use std::time::Duration;
use http::Uri;
use std::str::FromStr;
use super::spool_connection;
use std::sync::Arc;


pub struct SpoolMockConnection;

impl spool_connection::SpoolConnection for SpoolMockConnection {
    fn push(&self,message: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        
        Ok(())
    }

    fn pushToTopic(&self,message: Vec<u8>, topic: &String) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    fn consume(&self) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }

    fn consumeFromTopic(&self, topic: &String) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }
}


pub fn createMockConnection() -> Result<Arc<dyn spool_connection::SpoolConnection>, Box<dyn std::error::Error>> {
    Ok(Arc::new(SpoolMockConnection {}))
}