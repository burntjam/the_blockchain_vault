use crate::handler::message::{MessageHandler,MessageHandlerFactory};
use crate::message::{WebSocketMessage,HelloMessage,HelloResponse,ChallengeResponse,NetworkKeyResponse,TransactionMessage};
use super::socket::*;
use super::consensus::*;
use super::network::*;
use super::transaction::*;
use super::session::*;
use super::server::*;
use tokio::net::TcpStream;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::accept_async;
use futures::StreamExt;
use futures::FutureExt;
use futures::sink::SinkExt;
use tokio::net::TcpListener;

struct WebSocketSession {
    rt: tokio::runtime::Runtime,
    socket: Arc<Mutex<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>>>,
}

struct RPCWebSocket {
    session: Arc<WebSocketSession>,
}

impl RPCWebSocket {
    pub fn new(session: Arc<WebSocketSession>) -> Self {
        RPCWebSocket { session }
    }
}

impl ServerMessageSocket for RPCWebSocket {
    fn send(&self, msg: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let socket = self.session.socket.clone();
        self.session.rt.block_on(async {
            let mut socket = socket.lock().unwrap(); // Scope limited to this block
            socket.send(Message::text(msg)).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        })
    }
}


pub struct RPCWebSocketSession {
    account: String,
    listener: TcpListener,
    rt: tokio::runtime::Runtime,
}

struct RPCMessageProcessor {
    webSocketSession: Arc<WebSocketSession>,
    rpcWebSocket: Arc<RPCWebSocket>,
    session: Arc<Session>,
    factory: Arc<ServerMessageHandlerFactory>,
}

impl RPCMessageProcessor {
    pub fn processMessage(&self) {
        self.webSocketSession.rt.block_on(async {
            let mut ws = self.webSocketSession.socket.lock().unwrap();
            while let Some(message) = ws.next().await {
                match message {
                    Ok(msg) => {
                        if msg.is_text() {
                            let deserialized = deserialize_message(msg.to_text().unwrap()).unwrap();
                            self.factory.handleMessage(deserialized.clone()).unwrap().handleMessage(deserialized);
                        }
                    }
                    Err(e) => {
                        println!("Error processing message: {}", e);
                    }
                }
            }
        })
    }
}


fn deserialize_message(json_message: &str) -> Result<WebSocketMessage,Box<dyn std::error::Error>> {
    let deserialized: WebSocketMessage = serde_json::from_str(json_message)?;
    Ok(deserialized)
}

impl RPCWebSocketSession {
    pub fn new(account: String, address: String) -> Self {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let listener = rt.block_on(async{
            TcpListener::bind(&address.as_str()).await
        });
    
        RPCWebSocketSession{account,listener:listener.unwrap(),rt:rt}
    }

    pub fn process(&self) {
        self.rt.block_on(async {
            while let Ok((stream, _)) = self.listener.accept().await {
                tokio::spawn(handle_connection(self.account.clone(),stream));
            }
        })
    }
}

async fn handle_connection(account: String, stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    println!("New WebSocket connection established");

    tokio::spawn(handle_messages(account,ws_stream));
}

async fn handle_messages(account: String,mut ws: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>) {

    let websocketSession: Arc<WebSocketSession> = Arc::new(WebSocketSession{rt:tokio::runtime::Runtime::new().unwrap(),socket: Arc::new(Mutex::new(ws))});
    let rpcWebSocket: Arc<RPCWebSocket> = Arc::new(RPCWebSocket::new(websocketSession.clone()));
    let basicServerSession: Arc<BasicServerSession> = Arc::new(BasicServerSession::new(account, rpcWebSocket.clone()));
    let factory: Arc<ServerMessageHandlerFactory> = Arc::new(ServerMessageHandlerFactory::new(basicServerSession.clone()));
    let rpcMessageProcessor = RPCMessageProcessor{webSocketSession:websocketSession,rpcWebSocket,session:basicServerSession,factory};
    rpcMessageProcessor.processMessage()
}