use async_trait::async_trait;

#[async_trait]
pub trait SpoolConnection: Sync + Send {
    async fn push(&self,message: Vec<u8>) -> Result<(), Box<dyn std::error::Error>>;
    async fn pushToTopic(&self,message: Vec<u8>, topic: &String) -> Result<(), Box<dyn std::error::Error>>;
    async fn consume(&self) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>>;
    async fn consumeFromTopic(&self, topic: &String) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>>;
}