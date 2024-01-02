use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use futures::StreamExt;
use futures::FutureExt;
use futures::sink::SinkExt;
use config_lib::*;
use peer2peer_protocol::handler::server::grpc_server::*;

#[tokio::main]
async fn main() {
    let config = ChainConfig::new().unwrap();
    let rpc_web_socket_session = RPCWebSocketSession::new(config.peer_server.account,config.peer_server.address);
    rpc_web_socket_session.process();
}
