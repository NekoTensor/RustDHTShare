/*
    protocol.rs
    ----------------------------------------------------------------------------
    Defines the P2P messaging protocol using a strongly-typed enum.
    
    Features:
      - Each variant represents a different type of message exchanged between nodes.
      - Uses Serde for JSON serialization/deserialization.
    
    Developer Notes:
      - The protocol supports Join, Ping/Pong, Store, Lookup, FileRequest, and FileData messages.
      - Additional variants can be added as the project expands.
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
