use serde::{Deserialize, Serialize};

pub mod message_error;
pub mod network;
pub mod hand_shake;
pub mod transaction;
pub mod election;

pub use message_error::*;
pub use network::*;
pub use hand_shake::*;
pub use transaction::*;
pub use election::*;


// Use an enum if you have different kinds of messages
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "payload")]
pub enum WebSocketMessage {
    NetworkKeyQuery(network::NetworkKeyQuery),
    NetworkKeyResponse(network::NetworkKeyResponse),
    Hello(hand_shake::HelloMessage),
    HelloResponse(hand_shake::HelloResponse),
    Challenge(hand_shake::ChallengeMessage),
    ChallengeResponse(hand_shake::ChallengeResponse),
    Transaction(transaction::TransactionMessage),
    Election(election::ElectionMessage),
    NetworkHeartBeat(network::NetworkHeartBeat),
}


pub fn deserialize_message(json_message: &str) -> Result<WebSocketMessage,Box<dyn std::error::Error>> {
    let deserialized: WebSocketMessage = serde_json::from_str(json_message)?;
    Ok(deserialized)
}

pub fn deserialize_bin_message(bin_message: &Vec<u8>) -> Result<WebSocketMessage,Box<dyn std::error::Error>> {
    match String::from_utf8(bin_message.clone()) {
        Ok(string) => deserialize_message(&string),
        Err(e) => Err(Box::new(MessageError{message:"Failed failed to deserialize as the binary message is not a UTF8 string".to_string()})),
    }
}

pub fn serialize_message(web_socket_message: &WebSocketMessage) -> Result<String,Box<dyn std::error::Error>> {
    let serialized = serde_json::to_string(web_socket_message)?;
    Ok(serialized)
}


pub fn serialize_bin_message(web_socket_message: &WebSocketMessage) -> Result<Vec<u8>,Box<dyn std::error::Error>> {
    match serialize_message(web_socket_message) {
        Ok(string) => Ok(string.into_bytes()),
        Err(error) => Err(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    
    #[test]
    fn test_serialize_deserialize() {
        let network_heart_beat = WebSocketMessage::NetworkHeartBeat(NetworkHeartBeat 
            { cycle_id: "test".to_string(), timestamp: Utc::now(), network_slot: 1, network_election_slot: 1, network_election_publish_slot: 1, network_confirmation_slot: 1 });
        let str_msg = serialize_message(&network_heart_beat);
        let network_heart_beat_msg = deserialize_message(str_msg.unwrap().as_str()).unwrap();
        if let WebSocketMessage::NetworkHeartBeat(value1) = network_heart_beat_msg {
            if let WebSocketMessage::NetworkHeartBeat(value2) = network_heart_beat {
                assert_eq!(value1.cycle_id,value2.cycle_id);
                assert_eq!(value1.network_confirmation_slot,value2.network_confirmation_slot);
                assert_eq!(value1.network_election_publish_slot,value2.network_election_publish_slot);
                assert_eq!(value1.network_election_slot,value2.network_election_slot);
                assert_eq!(value1.network_slot,value2.network_slot);
                assert_eq!(value1.timestamp,value2.timestamp);
            } else {
                panic!("Test failed as values could not be converted correctly.");
            }
        } else {
            panic!("Test failed as values could not be converted correctly.");
        }
    }

    #[test]
    fn test_bin_serialize_deserialize() {
        let network_heart_beat = WebSocketMessage::NetworkHeartBeat(NetworkHeartBeat 
            { cycle_id: "test".to_string(), timestamp: Utc::now(), network_slot: 1, network_election_slot: 1, network_election_publish_slot: 1, network_confirmation_slot: 1 });
        let bin_msg = serialize_bin_message(&network_heart_beat);
        let network_heart_beat_msg = deserialize_bin_message(&bin_msg.unwrap()).unwrap();
        if let WebSocketMessage::NetworkHeartBeat(value1) = network_heart_beat_msg {
            if let WebSocketMessage::NetworkHeartBeat(value2) = network_heart_beat {
                assert_eq!(value1.cycle_id,value2.cycle_id);
                assert_eq!(value1.network_confirmation_slot,value2.network_confirmation_slot);
                assert_eq!(value1.network_election_publish_slot,value2.network_election_publish_slot);
                assert_eq!(value1.network_election_slot,value2.network_election_slot);
                assert_eq!(value1.network_slot,value2.network_slot);
                assert_eq!(value1.timestamp,value2.timestamp);
            } else {
                panic!("Test failed as values could not be converted correctly.");
            }
        } else {
            panic!("Test failed as values could not be converted correctly.");
        }
    }
}