//! # Replica Server
//!
//! The replica server maintains a read-only copy of the primary's data.
//! It handles:
//!
//! - Interactive CLI for GET operations and cluster management
//! - TCP server for receiving replication updates from primary
//! - Dynamic joining/leaving of the replication cluster
//!
//! ## Command Line Interface
//!
//! The replica accepts these commands:
//! - `GET <key>` - Retrieve a value from local storage
//! - `JOIN <host:port>` - Connect to primary and receive initial snapshot
//! - `EXIT` - Shutdown the replica
//!
//! ## Network Protocol
//!
//! The replica listens on a configurable port for replication updates:
//! - Receives `PUT <key> <value>` messages from primary
//! - Initial state transfer via snapshot during JOIN operation
//!
//! ## Usage
//!
//! ```bash
//! cargo run --bin replica [port]
//! ```
//!
//! The port parameter is optional (defaults to 8081).
//!
//! Example session:
//! ```text
//! Replica ready (port 8081)
//! Commands:
//! - GET <key>
//! - JOIN <host:port>
//! - EXIT
//! replica> JOIN 127.0.0.1:8080
//! Snapshot received
//! replica> GET hello
//! hello -> world
//! replica> EXIT
//! Goodbye!
//! ```

use replication_kv::protocol::Message;
use replication_kv::store::KVStore;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::io::{BufRead, BufReader, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

/// Main entry point for the replica server.
///
/// Parses command line arguments for port configuration, starts the TCP
/// server for receiving replication updates, then runs the interactive CLI.
fn main() {
    let replica_port = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "8081".to_string());
    let replica_addr = format!("127.0.0.1:{}", replica_port);
    println!("Replica ready (port {})", replica_port);

    let storage = Arc::new(KVStore::new());

    thread::spawn({
        let replica_addr = replica_addr.clone();
        let storage = storage.clone();
        move || {
            start_replica_server(&replica_addr, storage);
        }
    });

    let mut rl = DefaultEditor::new().expect("Failed to create readline editor");

    println!("Commands:");
    println!("- GET <key>");
    println!("- JOIN <host:port>");
    println!("- EXIT");

    loop {
        match rl.readline("replica> ") {
            Ok(line) => {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();

                match parts.as_slice() {
                    ["GET", key] => match storage.get(key) {
                        Some(value) => println!("{} -> {}", key, value),
                        None => println!("{} -> Not found", key),
                    },
                    ["JOIN", host_port] => {
                        let message = Message::Join {
                            replica_addr: replica_addr.clone(),
                        };
                        join_primary(host_port, &storage, &message);
                    }
                    ["EXIT"] => {
                        println!("Goodbye!");
                        break;
                    }
                    _ => {
                        println!("Invalid command. Use: GET <key>, JOIN <host:port>, or EXIT");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("EOF");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}

/// Start the TCP server for receiving replication updates.
///
/// Listens on the specified address and spawns a new thread for each
/// incoming connection from the primary server.
///
/// # Arguments
/// * `replica_addr` - Address to bind the server to (e.g., "127.0.0.1:8081")
/// * `storage` - Shared key-value store for applying updates
fn start_replica_server(replica_addr: &str, storage: Arc<KVStore>) {
    let listener = TcpListener::bind(replica_addr).expect("Failed to bind replica server");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let storage_clone = Arc::clone(&storage);
                thread::spawn(move || {
                    handle_connection(stream, storage_clone);
                });
            }
            Err(_) => {}
        }
    }
}

/// Handle a single replication update from the primary.
///
/// Reads one message from the stream and applies PUT operations to
/// the local storage. Other message types are ignored.
///
/// # Arguments
/// * `stream` - TCP connection from the primary
/// * `storage` - Local key-value store to update
fn handle_connection(mut stream: TcpStream, storage: Arc<KVStore>) {
    let mut buffer = [0; 1024];
    if let Ok(size) = stream.read(&mut buffer) {
        let data = String::from_utf8_lossy(&buffer[..size]);
        if let Some(message) = Message::parse(data.trim()) {
            match message {
                Message::Put { key, value } => {
                    storage.put(key, value);
                }
                _ => {
                    // Ignore other messages
                }
            }
        }
    }
}

/// Connect to the primary server and receive initial state snapshot.
///
/// Sends a JOIN message to the primary, then reads the complete state
/// snapshot line by line until SNAPSHOT_END is received. All PUT messages
/// are applied to local storage.
///
/// # Arguments
/// * `host_port` - Primary server address (e.g., "127.0.0.1:8080")
/// * `storage` - Local storage to populate with snapshot data
/// * `join_message` - JOIN message containing this replica's address
fn join_primary(host_port: &str, storage: &Arc<KVStore>, join_message: &Message) {
    match TcpStream::connect(host_port) {
        Ok(mut stream) => {
            if stream.write_all(join_message.format().as_bytes()).is_err() {
                return;
            }
            let mut reader = BufReader::new(stream);
            loop {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) => break,
                    Ok(_) => {
                        if let Some(message) = Message::parse(line.trim()) {
                            match message {
                                Message::Put { key, value } => {
                                    storage.put(key, value);
                                }
                                Message::SnapshotEnd => {
                                    println!("Snapshot received");
                                    return;
                                }
                                _ => {
                                    // Ignore other messages
                                }
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        }
        Err(_) => {
            println!("Failed to connect to {}", host_port);
        }
    }
}
