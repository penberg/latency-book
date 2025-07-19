//! # Key-Value Storage
//!
//! This module provides a thread-safe key-value store with interior mutability.
//! The store uses a `Mutex` internally to allow concurrent access from multiple
//! threads while maintaining data consistency.

use std::collections::HashMap;
use std::sync::Mutex;

/// A thread-safe key-value store with interior mutability.
///
/// This store allows multiple threads to safely read and write key-value pairs
/// without requiring external synchronization. All operations are atomic and
/// the store handles locking internally.
pub struct KVStore {
    data: Mutex<HashMap<String, String>>,
}

impl KVStore {
    /// Create a new empty key-value store.
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }

    /// Retrieve a value for the given key.
    ///
    /// Returns a cloned copy of the value if the key exists, or `None` if
    /// the key is not found. This operation is thread-safe.
    ///
    /// # Arguments
    /// * `key` - The key to look up
    ///
    /// # Returns
    /// * `Some(String)` - The value associated with the key
    /// * `None` - If the key does not exist
    pub fn get(&self, key: &str) -> Option<String> {
        self.data.lock().unwrap().get(key).cloned()
    }

    /// Store a key-value pair.
    ///
    /// If the key already exists, its value will be updated. This operation
    /// is thread-safe and atomic.
    ///
    /// # Arguments
    /// * `key` - The key to store
    /// * `value` - The value to associate with the key
    pub fn put(&self, key: String, value: String) {
        self.data.lock().unwrap().insert(key, value);
    }

    /// Get all key-value pairs as a vector.
    ///
    /// Returns a snapshot of all current key-value pairs. The returned
    /// vector contains cloned copies of the keys and values, so it can
    /// be safely used without holding any locks.
    ///
    /// # Returns
    /// Vector of (key, value) tuples representing all stored data
    pub fn keys(&self) -> Vec<(String, String)> {
        self.data
            .lock()
            .unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}
