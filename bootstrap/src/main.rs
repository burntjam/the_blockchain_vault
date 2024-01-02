use tokio::sync::{mpsc, Mutex};
use tokio::net::{UnixStream, UnixListener};
use tokio::io::{AsyncWriteExt, AsyncBufReadExt, BufReader};
use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};
use std::error::Error;
use std::sync::Arc;
use wasmtime::{Func, Store, Linker, Trap, Extern, Instance, ImportType, Val, ValType, Caller};
use reqwest;


#[derive(Debug, Serialize, Deserialize)]
struct Message {
    sender: String,
    content: String,
}

struct NamedUnixStream {
    name: String,
    stream: UnixStream,
}

struct WasmHandler {
    store: Store<()>,
    instance: Option<Instance>,
    callback: Arc<dyn Fn(i32) + Send + Sync>,
}

impl NamedUnixStream {
    async fn connect(path: &str) -> Result<Self, Box<dyn Error>> {
        let stream = UnixStream::connect(path).await?;
        Ok(Self {
            name: path.to_string(),
            stream,
        })
    }

    async fn bind(path: &str) -> Result<Self, Box<dyn Error>> {
        let listener = UnixListener::bind(path)?;
        let (stream, _) = listener.accept().await?;
        Ok(Self {
            name: path.to_string(),
            stream,
        })
    }
}

struct MessageSender {
    tx: mpsc::Sender<Message>,
    client: NamedUnixStream,
}

impl MessageSender {
    async fn new(client: NamedUnixStream, tx: mpsc::Sender<Message>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            tx,
            client,
        })
    }

    async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut reader = BufReader::new(&mut self.client.stream);
        let mut line = String::new();
        while reader.read_line(&mut line).await? > 0 {
            let message = Message {
                sender: self.client.name.clone(),
                content: line.clone(),
            };
            self.tx.send(message).await.unwrap();
            line.clear();
        }
        Ok(())
    }
}

struct MessageReceiver {
    rx: mpsc::Receiver<Message>,
    server: NamedUnixStream,
}

impl MessageReceiver {
    async fn new(server: NamedUnixStream, rx: mpsc::Receiver<Message>) -> Result<Self, Box<dyn Error>> {
        Ok(Self { rx, server })
    }

    async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        while let Some(message) = self.rx.recv().await {
            let buffer = serialize(&message).unwrap();
            if self.server.stream.write_all(&buffer).await.is_err() {
                break;
            }
        }
        Ok(())
    }
}

impl WasmHandler {
    fn new(stream: Arc<Mutex<Option<tokio::net::UnixStream>>>) -> WasmHandler {
        let callback = {
            let stream = Arc::clone(&stream);
            move |ptr: i32|  {
                let mut guard = stream.try_lock().unwrap();
                if let Some(stream) = &mut *guard {
                    let message = Message { sender: String::new(), content: String::new() };
                    let buffer = serialize(&message).unwrap();
                    if let Err(e) = stream.try_write(&buffer) {
                        eprintln!("Failed to send message: {}", e);
                    }
                }
            }
        };

        WasmHandler {
            store: Store::<()>::default(),
            instance: None,
            callback: Arc::new(callback),
        }
    }

    async fn load(&mut self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let response = reqwest::get(url).await?;
        let wasm_bytes = response.bytes().await?;
        let module = wasmtime::Module::new(&self.store.engine(), wasm_bytes.as_ref())?;
        let instance = wasmtime::Instance::new(&mut self.store, &module, &[])?;
        self.instance = Some(instance);

        // Initialize the module
        let start = self.get_func("_start").unwrap();
        start.call(&mut self.store,&[], &mut [])?;
        Ok(())
    }

    async fn register_callback(&self) -> Result<(), Box<dyn std::error::Error>> {
        //let register_callback = self.get_func("register_callback")?.get1::<usize>()?;
        //register_callback(self.callback as usize)?;
        Ok(())
    }

    fn receive_message(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let receive_message = self.get_func("receive_message").unwrap();
        receive_message.call(&mut self.store,&[], &mut [])?;
        Ok(())
    }

    fn get_func(&mut self, name: &str) -> Result<Func, Box<dyn std::error::Error>> {
        Ok(self.instance.as_ref().ok_or("No WASM instance loaded")?.get_func(&mut self.store, name).ok_or("Function not found")?)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <server_socket_path> <client_socket_path> <wasm_url>", args[0]);
        return Ok(());
    }

    let server_path = &args[1];
    let client_path = &args[2];
    let wasm_url = &args[3];

    let client = NamedUnixStream::connect(client_path).await?;
    let server = NamedUnixStream::bind(server_path).await?;

    let (tx, rx) = mpsc::channel(32);

    let mut sender = MessageSender::new(client, tx).await?;
    let mut receiver = MessageReceiver::new(server, rx).await?;

    let sender_handle = tokio::spawn(async move {
        sender.run().await.unwrap();
    });

    let receiver_handle = tokio::spawn(async move {
        receiver.run().await.unwrap();
    });

    sender_handle.await.unwrap();
    receiver_handle.await.unwrap();

    Ok(())
}
