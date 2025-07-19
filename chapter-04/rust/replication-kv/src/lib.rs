//! # Key-Value Replication System
//!
//! A simple distributed key-value store demonstrating database replication concepts
//! for low-latency systems. This library implements a primary-replica architecture
//! where a single primary server handles writes and replicates data to multiple
//! read-only replicas.
//!
//! ## System Architecture
//!
//! The system consists of two main components:
//!
//! - **Primary Server**: Accepts PUT/GET commands via interactive CLI and TCP connections
//!   from replicas. Maintains the authoritative copy of data and replicates changes.
//! - **Replica Servers**: Accept GET commands via CLI and receive replicated data from
//!   the primary. Can join/leave the cluster dynamically.
//!
//! ## Usage
//!
//! ### Starting the Primary
//! ```bash
//! cargo run --bin primary
//! ```
//!
//! The primary listens on `127.0.0.1:8080` and accepts these commands:
//! - `PUT <key> <value>` - Store a key-value pair and replicate to all replicas
//! - `GET <key>` - Retrieve a value for the given key
//! - `EXIT` - Shutdown the server
//!
//! ### Starting a Replica
//! ```bash
//! cargo run --bin replica [port]
//! ```
//!
//! The replica listens on the specified port (default 8081) and accepts:
//! - `GET <key>` - Retrieve a value from local storage
//! - `JOIN <host:port>` - Connect to primary and receive initial snapshot
//! - `EXIT` - Shutdown the replica
//!
//! ## Replication Protocol
//!
//! The system uses a simple line-based TCP protocol with newline-terminated messages:
//!
//! - `JOIN <replica_addr>\n` - Replica registers with primary
//! - `PUT <key> <value>\n` - Replicate a key-value update
//! - `SNAPSHOT_END\n` - Marks end of initial state transfer
//!
//! ## Example Workflow
//!
//! 1. Start primary: `cargo run --bin primary`
//! 2. Start replica: `cargo run --bin replica 8081`
//! 3. In replica CLI: `JOIN 127.0.0.1:8080`
//! 4. In primary CLI: `PUT hello world`
//! 5. In replica CLI: `GET hello` → Returns "world"
//!
//! ## Modules
//!
//! - [`store`] - Thread-safe key-value storage with interior mutability
//! - [`topology`] - Replica set management for tracking connected replicas
//! - [`protocol`] - Message types and parsing for network communication

pub mod protocol;
pub mod store;
pub mod topology;
