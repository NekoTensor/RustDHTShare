/*
    main.rs
    ----------------------------------------------------------------------------
    Entry point for the P2P File Sharing project.
    
    Features:
      - Parses command-line arguments to determine the node's mode.
      - Supports a bootstrap mode (server) and a node mode with subcommands for 
        storing and looking up key-value pairs in the DHT.
      - Utilizes Tokio for asynchronous networking.
    
    Developer Notes:
      - Extensively debugged with multiple scenarios.
      - Error handling and logging have been improved to trace issues during runtime.
      - Use Ctrl+C to gracefully shut down the application.
    ----------------------------------------------------------------------------
*/

use std::env;
use tokio::signal;
use tokio::net::TcpStream;
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use serde_json;

mod dht;
mod network;
mod file_manager;
mod protocol;

use protocol::Message;

#[tokio::main]
async fn main() {
    // Parse command-line arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [bootstrap|node]", args[0]);
        return;
    }
    let mode = &args[1];

    // Dispatch based on selected mode.
    match mode.as_str() {
        "bootstrap" => {
            // Bootstrap mode: starts the server to listen for incoming connections.
            println!("Starting bootstrap node on port 8080...");
            network::start_server(8080).await;
        },
        "node" => {
            // Node mode: can execute subcommands to store or lookup data.
            if args.len() == 2 {
                println!("Starting a regular node (joining network)...");
                network::join_network("127.0.0.1:8080").await;
            } else {
                match args[2].as_str() {
                    "store" => {
                        if args.len() < 5 {
                            eprintln!("Usage: {} node store <key> <value>", args[0]);
                            return;
                        }
                        let key = args[3].clone();
                        let value = args[4].clone();
                        store_key_value(key, value).await;
                    }
                    "lookup" => {
                        if args.len() < 4 {
                            eprintln!("Usage: {} node lookup <key>", args[0]);
                            return;
                        }
                        let key = args[3].clone();
                        lookup_key(key).await;
                    }
                    _ => {
                        eprintln!("Unknown subcommand: {}", args[2]);
                    }
                }
            }
        },
        _ => {
            eprintln!("Unknown mode. Use 'bootstrap' or 'node'");
        }
    }

    // Wait for a shutdown signal (Ctrl+C) to gracefully terminate the application.
    match signal::ctrl_c().await {
        Ok(_) => println!("Received shutdown signal, terminating..."),
        Err(e) => eprintln!("Error waiting for shutdown signal: {:?}", e),
    }
}

/// Connects to the bootstrap node and sends a Store command.
/// Debug logs and error messages are printed during the process.
async fn store_key_value(key: String, value: String) {
    match TcpStream::connect("127.0.0.1:8080").await {
        Ok(mut stream) => {
            // Construct the Store message.
            let msg = Message::Store { key, value };
            let msg_json = serde_json::to_string(&msg).unwrap();
            if let Err(e) = stream.write_all(msg_json.as_bytes()).await {
                eprintln!("Error sending store message: {:?}", e);
            }
            // Signal end-of-message with a newline.
            stream.write_all(b"\n").await.unwrap();

            // Read and deserialize the response.
            let mut reader = BufReader::new(stream);
            let mut response = String::new();
            reader.read_line(&mut response).await.unwrap();
            let resp_msg: Message = serde_json::from_str(&response).unwrap();
            println!("Store response: {:?}", resp_msg);
        }
        Err(e) => {
            eprintln!("Could not connect to bootstrap node: {:?}", e);
        }
    }
}

/// Connects to the bootstrap node and sends a Lookup command.
/// The returned response is printed to standard output.
async fn lookup_key(key: String) {
    match TcpStream::connect("127.0.0.1:8080").await {
        Ok(mut stream) => {
            // Construct the Lookup message.
            let msg = Message::Lookup { key };
            let msg_json = serde_json::to_string(&msg).unwrap();
            if let Err(e) = stream.write_all(msg_json.as_bytes()).await {
                eprintln!("Error sending lookup message: {:?}", e);
            }
            stream.write_all(b"\n").await.unwrap();

            // Read and parse the response.
            let mut reader = BufReader::new(stream);
            let mut response = String::new();
            reader.read_line(&mut response).await.unwrap();
            let resp_msg: Message = serde_json::from_str(&response).unwrap();
            println!("Lookup response: {:?}", resp_msg);
        }
        Err(e) => {
            eprintln!("Could not connect to bootstrap node: {:?}", e);
        }
    }
}
