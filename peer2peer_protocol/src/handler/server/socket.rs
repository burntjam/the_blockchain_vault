

pub trait ServerMessageSocket {
    fn send(&self, msg: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}