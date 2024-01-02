

pub trait ClientMessageConnection {
    fn send(&self, msg: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}