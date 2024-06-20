/// an asynchrouous client and server
/// coding along with the async-chat example in O'Reilly's Programming Rust
/// main goal of the example is to handle the backpressure well
/// backpressure:
/// -> a client having a slow connection or dropping out must never 
///    affect other clients ability to exchange messages
/// -> messages for clients who can't keep up are dropped by the server,
///    but clients get informed that their stream in incomplete
///
/// this library crate captures the entire chat protocol in two types:

use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod utils;

// FromClient represents the packets a client can send to the server
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum FromClient {
    // can ask to join a group and post messages to any group it has joined
    Join { group_name: Arc<String> },
    Post {
        group_name: Arc<String>,
        message: Arc<String>
    }
}

// FromServer represnts what the server can send back
// -> messages posted to some group and error messages
#[derive(Debug, Deserialize, Serialize, PartialEq)]
// the serde derives allow to convert messages to JSON and deserialize the received messages
pub enum FromServer {
    // -> send back messages posted to some group
    Message {
        // using a reference counted Arc<String> helps avoiding making copies of strings
        group_name: Arc<String>,
        message: Arc<String>
    },
    // -> send back error messages
    Error(String)
}

#[test]
fn test_from_client_json() {
    use std::sync::Arc;

    let from_client = FromClient::Post {
        group_name: Arc::new("Dogs".to_string()),
        message: Arc::new("Samoyeds Rock!".to_string()),
    };

    let json = serde_json::to_string(&from_client).unwrap();
    assert_eq!(
        json,
        r#"{"Post":{"group_name":"Dogs","message":"Samoyeds Rock!"}}"#
    );

    assert_eq!(
        serde_json::from_str::<FromClient>(&json).unwrap(),
        from_client
    );

}