//! # Primary Server
//!
//! The primary server is the authoritative source of truth in the replication system.
//! It handles:
//!
//! - Interactive CLI for PUT/GET operations
//! - TCP server for replica registration and initial state transfer
//! - Real-time replication of changes to all connected replicas
//!
//! ## Command Line Interface
//!
//! The primary accepts these commands:
//! - `PUT <key> <value>` - Store a key-value pair and replicate to all replicas
//! - `GET <key>` - Retrieve a value from local storage
//! - `EXIT` - Shutdown the server
//!
//! ## Network Protocol
//!
//! The primary listens on `127.0.0.1:8080` for replica connections:
//! - Replicas send `JOIN <replica_addr>` to register
//! - Primary responds with complete state snapshot
//! - Ongoing changes are pushed to all registered replicas
//!
//! ## Usage
//!
//! ```bash
//! cargo run --bin primary
//! ```
//!
//! Example session:
//! ```text
//! Primary server ready (port 8080)
//! Commands:
//! - PUT <key> <value>
//! - GET <key>
//! - EXIT
//! primary> PUT hello world
//! OK hello = world
//! primary> GET hello
//! hello -> world
//! primary> EXIT
//! Goodbye!
//! ```

use replication_kv::protocol::Message;
use replication_kv::store::KVStore;
use replication_kv::topology::ReplicaSet;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

/// Main entry point for the primary server.
///
/// Initializes the key-value store and replica set, starts the TCP server
/// in a background thread, then runs the interactive CLI in the main thread.
fn main() {
    println!("Primary server ready (port 8080)");

    let storage = Arc::new(KVStore::new());
    let replicas = Arc::new(ReplicaSet::new());

    thread::spawn({
        let storage = storage.clone();
        let replicas = replicas.clone();
        move || {
            start_primary_server(storage, replicas);
        }
    });

    let mut rl = DefaultEditor::new().expect("Failed to create readline editor");

    println!("Commands:");
    println!("  - PUT <key> <value>");
    println!("  - GET <key>");
    println!("  - EXIT");

    loop {
        match rl.readline("primary> ") {
            Ok(line) => {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                match parts.as_slice() {
                    ["PUT", key, value] => {
                        storage.put(key.to_string(), value.to_string());
                        let message = Message::Put {
                            key: key.to_string(),
                            value: value.to_string(),
                        };
                        broadcast(&replicas, &message);
                        println!("OK {} = {}", key, value);
                    }
                    ["GET", key] => match storage.get(key) {
                        Some(value) => println!("{} -> {}", key, value),
                        None => println!("{} -> Not found", key),
                    },
                    ["EXIT"] => {
                        println!("Goodbye!");
                        break;
                    }
                    _ => {
                        println!("Invalid command. Use: PUT <key> <value>, GET <key>, or EXIT");
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

/// Start the TCP server for handling replica connections.
///
/// Listens on 127.0.0.1:8080 and spawns a new thread for each incoming
/// connection. Each connection is expected to be a replica registration.
///
/// # Arguments
/// * `storage` - Shared key-value store for state snapshots
/// * `replicas` - Shared replica set for registration tracking
fn start_primary_server(storage: Arc<KVStore>, replicas: Arc<ReplicaSet>) {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn({
                    let storage = storage.clone();
                    let replicas = replicas.clone();
                    move || {
                        handle_connection(stream, storage, replicas);
                    }
                });
            }
            Err(_) => {
                println!("Failed to accept connection");
            }
        }
    }
}

/// Handle a single replica connection.
///
/// Reads the JOIN message from the replica, registers it in the replica set,
/// and sends the current state snapshot.
///
/// # Arguments
/// * `stream` - TCP connection from the replica
/// * `storage` - Key-value store for snapshot data
/// * `replicas` - Replica set for registration
fn handle_connection(mut stream: TcpStream, storage: Arc<KVStore>, replicas: Arc<ReplicaSet>) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let raw_msg = String::from_utf8_lossy(&buffer[..size]);
            if let Some(msg) = Message::parse(raw_msg.trim()) {
                match msg {
                    Message::Join { replica_addr } => {
                        replicas.register(replica_addr.clone());
                        send_snapshot(&mut stream, &storage);
                    }
                    _ => {
                        // Ignore other messages
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to read from connection: {}", e);
        }
    }
}

/// Send complete state snapshot to a newly joined replica.
///
/// Transmits all current key-value pairs followed by SNAPSHOT_END marker
/// to indicate the end of initial state transfer.
///
/// # Arguments
/// * `stream` - TCP stream to the replica
/// * `storage` - Key-value store containing current state
fn send_snapshot(stream: &mut TcpStream, storage: &Arc<KVStore>) {
    let entries = storage.keys();
    let snapshot = Message::Snapshot { entries };
    let _ = stream.write_all(snapshot.format().as_bytes());
}

/// Broadcast a message to all registered replicas.
///
/// Attempts to connect to each replica and send the message. Connection
/// failures are silently ignored to avoid blocking the primary.
///
/// # Arguments
/// * `replicas` - Set of replica addresses to send to
/// * `message` - Protocol message to transmit
fn broadcast(replicas: &Arc<ReplicaSet>, message: &Message) {
    for replica_addr in replicas.iter() {
        if let Ok(mut stream) = TcpStream::connect(&replica_addr) {
            let _ = stream.write_all(message.format().as_bytes());
        }
    }
}
