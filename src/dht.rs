/*
    dht.rs
    ----------------------------------------------------------------------------
    Implements a simple Distributed Hash Table (DHT) for the P2P file sharing system.
    
    Features:
      - Uses an in-memory HashMap protected by a Tokio Mutex for asynchronous access.
      - Provides methods to insert and lookup key-value pairs.
    
    Developer Notes:
      - The DHTEntry struct is defined as a placeholder for potential future metadata.
      - GLOBAL_DHT is declared as a global instance via lazy_static.
      - Debug statements (commented out) are available for deeper inspection during debugging.
    ----------------------------------------------------------------------------
*/

use std::collections::HashMap;
use tokio::sync::Mutex;
use lazy_static::lazy_static;

/// Represents an entry in the DHT.
/// Currently serves as a placeholder for future extensions.
#[derive(Debug, Clone)]
pub struct DHTEntry {
    pub key: String,
    pub value: String,
}

/// The DHT structure encapsulating a HashMap in a Mutex for concurrent access.
pub struct DHT {
    pub store: Mutex<HashMap<String, String>>,
}

impl DHT {
    /// Creates a new, empty DHT.
    pub fn new() -> Self {
        DHT {
            store: Mutex::new(HashMap::new()),
        }
    }

    /// Asynchronously inserts a key-value pair into the DHT.
    pub async fn insert(&self, key: String, value: String) {
        let mut map = self.store.lock().await;
        map.insert(key, value);
        // Uncomment for verbose debugging:
        // log::debug!("Inserted key-value pair into DHT.");
    }

    /// Asynchronously retrieves the value associated with a key, if present.
    pub async fn lookup(&self, key: &str) -> Option<String> {
        let map = self.store.lock().await;
        map.get(key).cloned()
    }
}

// Create a global DHT instance for use across the application.
lazy_static! {
    pub static ref GLOBAL_DHT: DHT = DHT::new();
}
