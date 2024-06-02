pub mod sandbox_error;
pub mod wasm_executor;
pub mod wasm_executor_manager;
pub mod wasm_handler;
pub mod wasm_handler_factory;
pub mod request_consumer;
pub mod http_request_consumer;

use tokio::sync::{mpsc, Mutex};
use tokio::net::{UnixStream, UnixListener};
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};
use std::error::Error;
use std::sync::Arc;
use wasmtime::{Func, Store, Linker, Trap, Extern, Instance, ImportType, Val, ValType, Caller};
use reqwest;
use sandbox_error::*;
use wasm_executor::*;
use wasm_executor_manager::*;
use wasm_handler::*;
use wasm_handler_factory::*;
use http_request_consumer::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    Ok(())
}
