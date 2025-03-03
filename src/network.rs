/*
    network.rs
    ----------------------------------------------------------------------------
    Handles all networking operations for the P2P system.
    
    Features:
      - Implements the bootstrap server that listens for incoming connections.
      - Processes messages (Store, Lookup, etc.) and responds appropriately.
      - Provides a client function (join_network) for nodes to connect to the bootstrap node.
    
    Developer Notes:
      - Uses Tokio's asynchronous TCP streams for high-performance networking.
      - Each incoming connection is handled in its own asynchronous task.
      - Extensive error handling and logging are included for robustness.
    ----------------------------------------------------------------------------
*/

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use crate::protocol::Message;
use serde_json;
use std::error::Error;

/// Starts the bootstrap node server on the specified port.
/// Continuously accepts incoming connections and spawns tasks to handle them.
pub async fn start_server(port: u16) {
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Server listening on {}", addr);

    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("New connection from {}", addr);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                eprintln!("Error handling connection from {}: {:?}", addr, e);
            }
        });
    }
}

/// Processes an incoming TCP connection by reading a JSON-encoded message,
/// handling it, and writing back an appropriate response.
async fn handle_connection(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let (reader, mut writer) = socket.split();
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();

    // Read a single line (one complete JSON message).
    buf_reader.read_line(&mut line).await?;
    let msg: Message = serde_json::from_str(&line)?;
    println!("Received message: {:?}", msg);

    // Process message based on its type.
    match msg {
        Message::Store { key, value } => {
            // Insert the key-value pair into the DHT.
            crate::dht::GLOBAL_DHT.insert(key.clone(), value.clone()).await;
            println!("Stored in DHT: {} -> {}", key, value);
            let reply = Message::Ack;
            let reply_json = serde_json::to_string(&reply)?;
            writer.write_all(reply_json.as_bytes()).await?;
            writer.write_all(b"\n").await?;
        },
        Message::Lookup { key } => {
            if let Some(found_value) = crate::dht::GLOBAL_DHT.lookup(&key).await {
                println!("Lookup success: {} -> {}", key, found_value);
                let reply = Message::Store { key, value: found_value };
                let reply_json = serde_json::to_string(&reply)?;
                writer.write_all(reply_json.as_bytes()).await?;
                writer.write_all(b"\n").await?;
            } else {
                println!("Lookup miss: {}", key);
                let reply = Message::Ack;
                let reply_json = serde_json::to_string(&reply)?;
                writer.write_all(reply_json.as_bytes()).await?;
                writer.write_all(b"\n").await?;
            }
        },
        // For all other messages, simply acknowledge receipt.
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
/// Prints the response from the bootstrap node.
pub async fn join_network(bootstrap_addr: &str) {
    match TcpStream::connect(bootstrap_addr).await {
        Ok(mut stream) => {
            println!("Connected to bootstrap node at {}", bootstrap_addr);
            let join_msg = Message::Join { node_id: "node1".to_string() };
            let msg_json = serde_json::to_string(&join_msg).unwrap();
            if let Err(e) = stream.write_all(msg_json.as_bytes()).await {
                eprintln!("Error sending join message: {:?}", e);
            }
            stream.write_all(b"\n").await.unwrap();
            let mut reader = BufReader::new(stream);
            let mut response = String::new();
            reader.read_line(&mut response).await.unwrap();
            let resp: Message = serde_json::from_str(&response).unwrap();
            println!("Received response: {:?}", resp);
        },
        Err(e) => {
            eprintln!("Could not connect to bootstrap node: {:?}", e);
        }
    }
}
