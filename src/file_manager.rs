/*
    file_manager.rs
    ----------------------------------------------------------------------------
    Provides file-related functionality for the P2P system.
    
    Features:
      - Splits files into fixed-size chunks for distributed transfer.
      - Computes SHA-256 hashes of files or chunks to ensure data integrity.
    
    Developer Notes:
      - These functions have been tested on various file sizes.
      - For large files, consider streaming the data instead of reading it all into memory.
    ----------------------------------------------------------------------------
*/

use std::fs::File;
use std::io::{self, Read};
use sha2::{Sha256, Digest};
use std::path::Path;

/// Splits a file into chunks of the specified size.
/// Returns a vector of byte vectors, each representing a chunk.
/// 
/// # Arguments
/// * `path` - Path to the file to be split.
/// * `chunk_size` - The size (in bytes) of each chunk.
/// 
/// # Errors
/// Returns an `io::Error` if the file cannot be read.
pub fn split_file(path: &Path, chunk_size: usize) -> io::Result<Vec<Vec<u8>>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut chunks = Vec::new();
    for chunk in buffer.chunks(chunk_size) {
        chunks.push(chunk.to_vec());
    }
    Ok(chunks)
}

/// Computes the SHA-256 hash of the provided data slice.
/// Returns the hash as a hexadecimal string.
pub fn compute_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}
