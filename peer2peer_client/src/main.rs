use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::connect_async;
use futures_util::sink::SinkExt;  // Import SinkExt to get the send() method
use futures_util::stream::StreamExt;   // Import StreamExt for the next() method
use config_lib::*;
use peer2peer_protocol::handler::client::grpc_client::*;

#[tokio::main]
async fn main() {
    let config = ChainConfig::new().unwrap();
    let rpcWebSocketSession = RPCWebSocketSession::new(config.peer_client.account,config.peer_client.master);
    rpcWebSocketSession.process();
}
