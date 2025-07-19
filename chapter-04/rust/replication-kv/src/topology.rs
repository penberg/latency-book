//! # Replica Topology Management
//!
//! This module manages the set of replicas connected to the primary server.
//! It provides thread-safe operations for registering replicas and iterating
//! over them for replication purposes.

use std::sync::Mutex;

/// A thread-safe collection of replica addresses.
///
/// The ReplicaSet maintains a list of replica server addresses that have
/// registered with the primary. It uses interior mutability to allow
/// concurrent access from multiple threads handling replica connections
/// and replication operations.
pub struct ReplicaSet {
    replicas: Mutex<Vec<String>>,
}

impl ReplicaSet {
    /// Create a new empty replica set.
    pub fn new() -> Self {
        Self {
            replicas: Mutex::new(Vec::new()),
        }
    }

    /// Register a new replica with the set.
    ///
    /// Adds the replica address to the set of known replicas. The address
    /// should be in the format "host:port" and represent the listening
    /// address where the replica can receive replication updates.
    ///
    /// # Arguments
    /// * `replica_addr` - The network address of the replica server
    pub fn register(&self, replica_addr: String) {
        self.replicas.lock().unwrap().push(replica_addr);
    }

    /// Get a snapshot of all replica addresses.
    ///
    /// Returns a cloned vector of all currently registered replica addresses.
    /// This can be used to iterate over replicas for sending replication
    /// updates without holding the internal lock.
    ///
    /// # Returns
    /// Vector of replica addresses as strings
    pub fn iter(&self) -> Vec<String> {
        self.replicas.lock().unwrap().clone()
    }
}
