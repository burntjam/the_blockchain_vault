use crate::StoreSession;
use std::sync::Mutex;


pub trait StoreSessionFactory: Sync + Send {
    fn createSession(&self) -> Result<Box<Mutex<dyn StoreSession>>, Box<dyn std::error::Error>>;
}