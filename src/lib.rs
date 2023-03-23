use std::collections::HashMap;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
pub struct KvStore {
    map: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

impl KvStore {
    /// Creates a new `KvStore`
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Inserts the given value for the given key
    ///
    /// If the key already exists, the previous value will be replaced.
    pub fn set(&mut self, key: String, value: String) {
        println!("Setting '{}' to '{}'", key, value);
        self.map.insert(key, value);
    }

    /// Gets the string value for a given key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Option<String> {
        println!("Getting value for '{}'", key);
        self.map.get(&key).cloned()
    }

    /// Removes the given key.
    pub fn remove(&mut self, key: String) {
        println!("Removed value for '{}'", key);
        self.map.remove(&key);
    }
}
