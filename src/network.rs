/*
    network.rs
    ----------------------------------------------------------------------------
    Handles networking operations for RustDHTShare.
    
    - Listens for incoming TCP connections on the bootstrap node.
    - Processes JSON-encoded messages (Store, Lookup, Join, etc.).
    - Uses Tokio for asynchronous I/O and logs all significant events.
    
    Debugged thoroughly to ensure robust error handling and clarity.
    ----------------------------------------------------------------------------
*/

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use crate::protocol::Message;
use serde_json;
use std::error::Error;
use log::{info, error};

/// Starts the bootstrap node server on the given port.
/// Continuously accepts incoming connections and spawns asynchronous tasks.
pub async fn start_server(port: u16) {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Server listening on {}", addr);

    loop {
        match listener.accept().await {
            Ok((socket, addr)) => {
                info!("New connection from {}", addr);
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(socket).await {
                        error!("Error handling connection from {}: {:?}", addr, e);
                    }
                });
            },
            Err(e) => error!("Error accepting connection: {:?}", e),
        }
    }
}

/// Processes an incoming TCP connection.
/// Reads one JSON message, processes it according to its type, and writes a response.
async fn handle_connection(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let (reader, mut writer) = socket.split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    // Read a full line representing a complete JSON message.
    buf_reader.read_line(&mut line).await?;
    let msg: Message = serde_json::from_str(&line)?;
    info!("Received message: {:?}", msg);

    // Process the message based on its type.
    match msg {
        Message::Store { key, value } => {
            crate::dht::GLOBAL_DHT.insert(key.clone(), value.clone()).await;
            info!("Stored in DHT: {} -> {}", key, value);
            let reply = Message::Ack;
            let reply_json = serde_json::to_string(&reply)?;
            writer.write_all(reply_json.as_bytes()).await?;
            writer.write_all(b"\n").await?;
        },
        Message::Lookup { key } => {
            if let Some(found_value) = crate::dht::GLOBAL_DHT.lookup(&key).await {
                info!("Lookup success: {} -> {}", key, found_value);
                let reply = Message::Store { key, value: found_value };
                let reply_json = serde_json::to_string(&reply)?;
                writer.write_all(reply_json.as_bytes()).await?;
                writer.write_all(b"\n").await?;
            } else {
                info!("Lookup miss: {}", key);
                let reply = Message::Ack;
                let reply_json = serde_json::to_string(&reply)?;
                writer.write_all(reply_json.as_bytes()).await?;
                writer.write_all(b"\n").await?;
            }
        },
        // For all other message types, simply acknowledge receipt.
        _ => {
            let reply = Message::Ack;
            let reply_json = serde_json::to_string(&reply)?;
            writer.write_all(reply_json.as_bytes()).await?;
            writer.write_all(b"\n").await?;
        },
    }
    Ok(())
}

/// Connects to the bootstrap node and sends a Join message.
/// Logs the response received from the bootstrap node.
pub async fn join_network(bootstrap_addr: &str) {
    match TcpStream::connect(bootstrap_addr).await {
        Ok(mut stream) => {
            info!("Connected to bootstrap node at {}", bootstrap_addr);
            let join_msg = Message::Join { node_id: "node1".to_string() };
            let msg_json = serde_json::to_string(&join_msg).unwrap();
            if let Err(e) = stream.write_all(msg_json.as_bytes()).await {
                error!("Error sending join message: {:?}", e);
            }
            if let Err(e) = stream.write_all(b"\n").await {
                error!("Error sending newline: {:?}", e);
            }
            let mut reader = BufReader::new(stream);
            let mut response = String::new();
            if let Err(e) = reader.read_line(&mut response).await {
                error!("Error reading response: {:?}", e);
            }
            match serde_json::from_str::<Message>(&response) {
                Ok(resp) => info!("Received response: {:?}", resp),
                Err(e) => error!("Failed to parse response: {:?}", e),
            }
        },
        Err(e) => {
            error!("Could not connect to bootstrap node: {:?}", e);
        }
    }
}
