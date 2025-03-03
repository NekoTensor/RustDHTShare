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
    /// Join: Sent by a node to join the network.
    Join { node_id: String },

    /// Ping: A heartbeat message to verify node availability.
    Ping,

    /// Pong: Response to a Ping message.
    Pong,

    /// Store: Instructs the receiver to store a key-value pair in the DHT.
    /// The response might echo the stored pair.
    Store { key: String, value: String },

    /// Lookup: Requests the value associated with a key in the DHT.
    /// The response should include the value if found.
    Lookup { key: String },

    /// FileRequest: Requests a file or a file chunk.
    FileRequest { file_id: String },

    /// FileData: Carries the data of a requested file or chunk.
    FileData { file_id: String, data: Vec<u8> },

    /// Ack: Acknowledgment message used as a default reply.
    Ack,
}
