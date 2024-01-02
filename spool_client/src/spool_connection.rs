

pub trait SpoolConnection {
    fn push(&self,message: Vec<u8>) -> Result<(), Box<dyn std::error::Error>>;
    fn pushToTopic(&self,message: Vec<u8>, topic: &String) -> Result<(), Box<dyn std::error::Error>>;
    fn consume(&self) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>>;
    fn consumeFromTopic(&self, topic: &String) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>>;
}