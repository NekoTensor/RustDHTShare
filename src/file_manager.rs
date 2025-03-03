/*
    file_manager.rs
    ----------------------------------------------------------------------------
    Provides file-related utilities for RustDHTShare.
    
    - Splits files into fixed-size chunks for distributed transfer.
    - Computes SHA-256 hashes for data integrity verification.
    
    Developer Notes:
      - Functions have been tested on files of various sizes.
      - For extremely large files, consider using a streaming approach.
    ----------------------------------------------------------------------------
*/

use std::fs::File;
use std::io::{self, Read};
use sha2::{Sha256, Digest};
use std::path::Path;

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

]pub fn compute_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}
