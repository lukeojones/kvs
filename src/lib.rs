use std::collections::HashMap;
use failure::Error;
use std::path::{PathBuf};
use std::result;

pub type Result<T> = result::Result<T, Error>;

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
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        println!("Setting '{}' to '{}'", key, value);
        self.map.insert(key, value);
        Ok(())
    }

    /// Gets the string value for a given key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        println!("Getting value for '{}'", key);
        Ok(self.map.get(&key).cloned())
    }

    /// Removes the given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        println!("Removed value for '{}'", key);
        self.map.remove(&key);
        Ok(())
    }

    /// Opens a KV Store from disk
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        panic!("This is ground control to major Tom!")
    }
}
