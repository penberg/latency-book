//! # Replication Protocol
//!
//! This module defines the wire protocol for communication between primary and replica
//! servers in the key-value replication system. The protocol is line-based using TCP,
//! with each message terminated by a newline character (`\n`).
//!
//! ## Message Format
//!
//! All messages follow the format: `<COMMAND> [arguments...]\n`
//!
//! ### Protocol Messages
//!
//! | Message | Direction | Format | Purpose |
//! |---------|-----------|--------|---------|
//! | `JOIN` | Replica → Primary | `JOIN <replica_addr>\n` | Register replica with primary |
//! | `PUT` | Primary → Replica | `PUT <key> <value>\n` | Replicate key-value update |
//! | `SNAPSHOT_END` | Primary → Replica | `SNAPSHOT_END\n` | Mark end of initial state transfer |
//!
//! ## Protocol Flow
//!
//! ### Initial Replica Registration
//! ```text
//! Replica                Primary
//!    |                      |
//!    | JOIN 127.0.0.1:8081  |
//!    |--------------------->|  (1) Replica registers
//!    |                      |
//!    | PUT key1 value1      |
//!    |<---------------------|  (2) Primary sends snapshot
//!    | PUT key2 value2      |
//!    |<---------------------|
//!    | SNAPSHOT_END         |
//!    |<---------------------|  (3) Snapshot complete
//! ```
//!
//! ### Ongoing Replication
//! ```text
//! Primary               Replica
//!    |                     |
//!    | PUT key3 value3     |
//!    |-------------------->|  (1) Primary replicates new data
//!    |                     |
//! ```
//!
//! ## Error Handling
//!
//! - Invalid message formats are silently ignored
//! - Network errors cause connection termination
//! - Partial reads are handled by buffering until complete lines are received

/// Represents a message in the replication protocol.
///
/// Each variant corresponds to a specific message type that can be sent
/// between primary and replica servers over TCP connections.
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    /// Replicate a key-value pair from primary to replica.
    ///
    /// Format: `PUT <key> <value>\n`
    ///
    /// This message is sent to all connected replicas
    /// whenever a new key-value pair is stored on the primary.
    Put { key: String, value: String },

    /// Register a replica with the primary server.
    ///
    /// Format: `JOIN <replica_addr>\n`
    ///
    /// Sent by replica to primary when joining the cluster. The replica_addr
    /// should be the listening address of the replica server for receiving
    /// replication updates.
    Join { replica_addr: String },

    /// Send complete state snapshot to a newly joined replica.
    ///
    /// This is not a single wire message, but represents the logical concept
    /// of sending all current key-value pairs followed by SNAPSHOT_END.
    /// Used internally for state transfer.
    Snapshot { entries: Vec<(String, String)> },

    /// Mark the end of initial state transfer.
    ///
    /// Format: `SNAPSHOT_END\n`
    ///
    /// Sent by primary to replica after all existing key-value pairs
    /// have been transmitted during initial registration.
    SnapshotEnd,
}

impl Message {
    /// Parse a line of text into a protocol message.
    ///
    /// # Arguments
    /// * `input` - A string slice containing a single line of protocol text
    ///
    /// # Returns
    /// * `Some(Message)` if the input is a valid protocol message
    /// * `None` if the input cannot be parsed
    pub fn parse(input: &str) -> Option<Message> {
        let input = input.trim();
        if input == "SNAPSHOT_END" {
            return Some(Message::SnapshotEnd);
        }
        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts.as_slice() {
            ["PUT", key, value] => Some(Message::Put {
                key: key.to_string(),
                value: value.to_string(),
            }),
            ["JOIN", replica_addr] => Some(Message::Join {
                replica_addr: replica_addr.to_string(),
            }),
            _ => None,
        }
    }

    /// Format a message to wire format.
    ///
    /// Converts the message to its string representation suitable for
    /// transmission over TCP. All messages are terminated with `\n`.
    ///
    /// # Returns
    /// String representation of the message with newline termination
    pub fn format(&self) -> String {
        match self {
            Message::Put { key, value } => format!("PUT {} {}\n", key, value),
            Message::Join { replica_addr } => format!("JOIN {}\n", replica_addr),
            Message::Snapshot { entries } => {
                let mut ret = String::new();
                for (key, value) in entries {
                    ret.push_str(&format!("PUT {} {}\n", key, value));
                }
                ret.push_str("SNAPSHOT_END\n");
                ret
            }
            Message::SnapshotEnd => "SNAPSHOT_END\n".to_string(),
        }
    }
}
