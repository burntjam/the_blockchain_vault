
pub mod spool_connection;
pub mod spool_connection_grpc;
pub mod spool_connection_mock;
pub mod spool_errors;

pub use spool_connection::*;
pub use spool_connection_grpc::*;
pub use spool_connection_mock::*;
pub use spool_errors::*;


pub struct SpoolSession<T: SpoolConnection> {
    spool_connection: T,
}

impl<T: SpoolConnection> SpoolSession<T> {
    pub fn new(spool_connection: T) -> Self {
        SpoolSession { spool_connection }
    }

    fn push(&self,message: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        self.spool_connection.push(message)
    }

    fn consume(&self) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
        self.spool_connection.consume()
    }
}