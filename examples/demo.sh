#!/usr/bin/env bash

# Start the bootstrap node in the background
echo "Starting bootstrap node..."
cargo run -- bootstrap &
BOOTSTRAP_PID=$!

# Wait a moment for the bootstrap node to start
sleep 2

echo "Storing a key-value pair in the DHT..."
cargo run -- node store myKey "HelloWorldValue"

echo "Looking up the key in the DHT..."
cargo run -- node lookup myKey

# Cleanup
kill $BOOTSTRAP_PID
