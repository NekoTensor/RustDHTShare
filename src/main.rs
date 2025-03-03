/*
    main.rs
    ----------------------------------------------------------------------------
    Entry point for RustDHTShare: A Distributed File Sharing Platform in Rust.
    
    Features:
      - Command-line argument parsing via Clap.
      - Supports both bootstrap (server) and node (client) modes.
      - Subcommands for joining the network, storing key-value pairs, and performing lookups.
      - Logging for debugging and traceability.
    
    Debugged extensively with various scenarios.
    ----------------------------------------------------------------------------
*/

use clap::{Parser, Subcommand};
use log::{info, error};
use tokio::net::TcpStream;
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use serde_json;

mod dht;
mod network;
mod file_manager;
mod protocol;

use protocol::Message;

/// RustDHTShare: A Distributed File Sharing Platform in Rust.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Command to run.
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run as the bootstrap node (server mode).
    Bootstrap {
        /// Port to listen on (default: 8080).
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
    /// Run as a regular node (client mode).
    Node {
        /// Subcommands for node operations.
        #[command(subcommand)]
        action: Option<NodeAction>,
    },
}

#[derive(Subcommand)]
enum NodeAction {
    /// Join the network.
    Join,
    /// Store a key-value pair.
    Store {
        /// The key to store.
        key: String,
        /// The corresponding value.
        value: String,
    },
    /// Lookup a value by key.
    Lookup {
        /// The key to lookup.
        key: String,
    },
}

#[tokio::main]
async fn main() {
    // Initialize logging using env_logger. Configure log level via the RUST_LOG env var.
    env_logger::init();
    
    // Parse CLI arguments.
    let cli = Cli::parse();

    match cli.command {
        Commands::Bootstrap { port } => {
            info!("Starting bootstrap node on port {}...", port);
            network::start_server(port).await;
        },
        Commands::Node { action } => {
            match action {
                // If no subcommand is provided, join the network by default.
                None | Some(NodeAction::Join) => {
                    info!("Starting node: joining network at 127.0.0.1:8080...");
                    network::join_network("127.0.0.1:8080").await;
                },
                Some(NodeAction::Store { key, value }) => {
                    info!("Storing key-value pair: {} -> {}", key, value);
                    store_key_value(key, value).await;
                },
                Some(NodeAction::Lookup { key }) => {
                    info!("Looking up key: {}", key);
                    lookup_key(key).await;
                },
            }
        },
    }
}

/// Connects to the bootstrap node and sends a Store command.
/// Logs detailed error information for debugging.
async fn store_key_value(key: String, value: String) {
    match TcpStream::connect("127.0.0.1:8080").await {
        Ok(mut stream) => {
            let msg = Message::Store { key, value };
            let msg_json = serde_json::to_string(&msg).unwrap();
            if let Err(e) = stream.write_all(msg_json.as_bytes()).await {
                error!("Error sending store message: {:?}", e);
                return;
            }
            if let Err(e) = stream.write_all(b"\n").await {
                error!("Error sending newline: {:?}", e);
                return;
            }
            let mut reader = BufReader::new(stream);
            let mut response = String::new();
            if let Err(e) = reader.read_line(&mut response).await {
                error!("Error reading response: {:?}", e);
                return;
            }
            match serde_json::from_str::<Message>(&response) {
                Ok(resp_msg) => info!("Store response: {:?}", resp_msg),
                Err(e) => error!("Failed to parse response: {:?}", e),
            }
        },
        Err(e) => error!("Could not connect to bootstrap node: {:?}", e),
    }
}

/// Connects to the bootstrap node and sends a Lookup command.
/// Logs errors and prints the response.
async fn lookup_key(key: String) {
    match TcpStream::connect("127.0.0.1:8080").await {
        Ok(mut stream) => {
            let msg = Message::Lookup { key };
            let msg_json = serde_json::to_string(&msg).unwrap();
            if let Err(e) = stream.write_all(msg_json.as_bytes()).await {
                error!("Error sending lookup message: {:?}", e);
                return;
            }
            if let Err(e) = stream.write_all(b"\n").await {
                error!("Error sending newline: {:?}", e);
                return;
            }
            let mut reader = BufReader::new(stream);
            let mut response = String::new();
            if let Err(e) = reader.read_line(&mut response).await {
                error!("Error reading response: {:?}", e);
                return;
            }
            match serde_json::from_str::<Message>(&response) {
                Ok(resp_msg) => info!("Lookup response: {:?}", resp_msg),
                Err(e) => error!("Failed to parse response: {:?}", e),
            }
        },
        Err(e) => error!("Could not connect to bootstrap node: {:?}", e),
    }
}
