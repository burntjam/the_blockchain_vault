
use std::collections::HashMap;
use std::sync::{Mutex, Arc,mpsc};
use tonic::{transport::Server, Request, Response};
use spool::spooler_server::{Spooler, SpoolerServer};
use spool::{SpoolRequest, SpoolSubscriber, SpoolResponse};
use std::thread;
use std::time::Duration;


pub mod spool {
    tonic::include_proto!("spool");
}

#[derive(Debug,Clone)]
struct TopicCache {
    entries: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl TopicCache {
    pub fn new() -> TopicCache {
        TopicCache{
            entries: Arc::new(Mutex::new(Vec::new()))
        }
    }
    pub fn poll(
        &self) -> 
        Result<SpoolResponse, Box<dyn std::error::Error>> {
        let mut entries = self.entries.lock().unwrap();
        if entries.len() > 0 {
            let entriesCopy = entries.clone();
            entries.truncate(0);
            return Ok(SpoolResponse{
                body: entriesCopy,
            })
        }
        thread::sleep(Duration::from_millis(500));
        Ok(SpoolResponse{
            body: vec![]
        })
    }

    pub fn push(
        &self,
        entry: Vec<u8>) -> 
        Result<(), Box<dyn std::error::Error>> {
        let mut entries = self.entries.lock().unwrap();
        entries.push(entry);
        Ok({})
    }

}

#[derive(Debug)]
struct SpoolerImpl<> {
    cache: Arc<Mutex<HashMap<String,TopicCache>>>,
}

impl SpoolerImpl {
    fn new() -> SpoolerImpl {
        SpoolerImpl{
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    fn getCache(
        &self,
        topic: String,) -> Result<Option<TopicCache>, Box<dyn std::error::Error>> {
            let mut entries = self.cache.lock().unwrap();
            if entries.contains_key(&topic) != true {
                entries.insert(topic.clone(), TopicCache::new());
            }
            let result: Option<TopicCache> = entries.get(&topic).cloned();
            Ok(result)
        }
}

#[tonic::async_trait]
impl Spooler for SpoolerImpl {
    async fn push(
        &self, 
        request: Request<SpoolRequest>,
    ) -> Result<Response<()>, tonic::Status> {
        let message = request.into_inner();
        let topic = message.topic;
        let body = message.body;
        let cache = self.getCache(topic).unwrap().unwrap();
        cache.push(body).unwrap();
        //cache.push()
        Ok(Response::new({}))
        }

    async fn consume(
        &self,
        request: tonic::Request<SpoolSubscriber>,
    ) -> Result<Response<SpoolResponse>, tonic::Status> {
        let topic = request.into_inner().topic;
        let cache = self.getCache(topic).unwrap().unwrap();
        let response = cache.poll().unwrap();
        Ok(Response::new(response))
        }

}


pub struct SpoolerService;

impl SpoolerService {
    pub fn new() -> SpoolerService {
        SpoolerService{}
    }
    
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = "0.0.0.0:50080".parse().unwrap();
        let spoolerImpl = SpoolerImpl{
            cache: Arc::new(Mutex::new(HashMap::new())),
        };
        Server::builder()
            .add_service(SpoolerServer::new(spoolerImpl))
            .serve(addr)
            .await?;

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache1() {
        let cache1 = TopicCache::new();
        cache1.push(vec![1,2,3,4,5,6]);
        cache1.push(vec![2,2,2,2,2,2]);
        let entries = cache1.poll().unwrap();
        let values = entries.body;
        assert_eq!(values.len(), 2);

        let entries2 = cache1.poll().unwrap();
        let values2 = entries2.body;
        assert_eq!(values2.len(), 0);
    }

    #[tokio::test]
    async fn test_topic_cache1() {
        let spoolerImpl = SpoolerImpl::new();
        let request1 = SpoolRequest{
            topic: String::from("test1"),
            body: vec![1,2,3,4,5,6],
        };
        spoolerImpl.push(tonic::Request::new(request1)).await.unwrap();
        let request2 = SpoolRequest{
            topic: String::from("test1"),
            body: vec![1,2,3,4,5,6],
        };
        spoolerImpl.push(tonic::Request::new(request2)).await.unwrap();
        let request3 = SpoolRequest{
            topic: String::from("test2"),
            body: vec![1,2,3,4,5,6],
        };
        spoolerImpl.push(tonic::Request::new(request3)).await.unwrap();
        let request4 = SpoolRequest{
            topic: String::from("test2"),
            body: vec![1,2,3,4,5,6],
        };
        spoolerImpl.push(tonic::Request::new(request4)).await.unwrap();


        let request5: SpoolSubscriber = SpoolSubscriber{
            topic: String::from("test1"),
            timeout: 10
        };
        let response1 = spoolerImpl.consume(tonic::Request::new(request5.clone())).await.unwrap();
        let inner_response1 = response1.into_inner();
        assert_eq!(inner_response1.body.len(), 2);

        let response2 = spoolerImpl.consume(tonic::Request::new(request5.clone())).await.unwrap();
        let inner_response2 = response2.into_inner();
        assert_eq!(inner_response2.body.len(), 0);
    }
}