/*
    protocol.rs
    ----------------------------------------------------------------------------
    Defines the messaging protocol for RustDHTShare.
    
    - Uses a strongly-typed enum to represent all message variants.
    - JSON serialization/deserialization is handled via Serde.
    
    Developer Notes:
      - Supports Join, Ping, Pong, Store, Lookup, FileRequest, FileData, and Ack messages.
      - Easily extensible as new features are added.
    ----------------------------------------------------------------------------
*/

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    
    Join { node_id: String },

    Ping,

    Pong,

    Store { key: String, value: String },

    Lookup { key: String },

    FileRequest { file_id: String },

    FileData { file_id: String, data: Vec<u8> },

    Ack,
}
