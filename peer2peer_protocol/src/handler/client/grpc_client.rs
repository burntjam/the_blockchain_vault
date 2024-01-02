use crate::handler::message::{MessageHandler,MessageHandlerFactory};
use crate::message::{WebSocketMessage,HelloMessage,HelloResponse,ChallengeResponse,NetworkKeyResponse,TransactionMessage};
use super::connection::*;
use super::consensus::*;
use super::network::*;
use super::transaction::*;
use super::session::*;
use super::client::*;
use tokio::net::TcpStream;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::sink::SinkExt;
use tokio_tungstenite::connect_async;
use futures_util::stream::StreamExt;

struct WebSocketSession {
    rt: tokio::runtime::Runtime,
    socket: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
}

struct RPCWebSocketConnection {
    session: Arc<WebSocketSession>,
}

impl RPCWebSocketConnection {
    pub fn new(session: Arc<WebSocketSession>) -> Self {
        RPCWebSocketConnection { session }
    }
}

impl ClientMessageConnection for RPCWebSocketConnection {
    fn send(&self, msg: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let socket = self.session.socket.clone();
        self.session.rt.block_on(async {
            let mut socket = socket.lock().unwrap(); // Scope limited to this block
            socket.send(Message::text(msg)).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
        })
    }
}


pub struct RPCWebSocketSession {
    session: Arc<WebSocketSession>,
    rt: tokio::runtime::Runtime,
    factory: ClientMessageHandlerFactory,
}

fn deserialize_message(json_message: &str) -> Result<WebSocketMessage,Box<dyn std::error::Error>> {
    let deserialized: WebSocketMessage = serde_json::from_str(json_message)?;
    Ok(deserialized)
}

impl RPCWebSocketSession {
    pub fn new(account: String, url: String) -> Self {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let (mut socket, response) = rt.block_on(async {
            connect_async(url).await.expect("Failed to connect")
        });
        let web_socket_session = Arc::new(
            WebSocketSession{
                rt: rt,
                socket: Arc::new(Mutex::new(socket)),
            });
        
        let connection = Arc::new(RPCWebSocketConnection::new(web_socket_session.clone()));
        let session = Arc::new(BasicSession::new(account.clone(), connection.clone()));
        let factory = ClientMessageHandlerFactory::new(session.clone());
        let response = RPCWebSocketSession { session: web_socket_session.clone(), rt: tokio::runtime::Runtime::new().unwrap(), factory: factory};
        let message = WebSocketMessage::Hello(HelloMessage{account_id: account.clone()});
        response.processMessage(message);
        return response;
    }

    pub fn process(&self) {
        self.rt.block_on(async {
            let socket_clone = self.session.socket.clone();
            let mut socket_guard = socket_clone.lock().unwrap();

            while let Some(message) = socket_guard.next().await {
                let msg = message.expect("Error reading message");
                if msg.is_text() {
                    let deserialized = deserialize_message(msg.to_text().unwrap()).unwrap();
                    self.processMessage(deserialized);
                }
            }
        })
    }

    fn processMessage(&self,message: WebSocketMessage) {
        let handler = self.factory.handleMessage(message.clone()).unwrap();
        let result = handler.handleMessage(message.clone());
    }
}