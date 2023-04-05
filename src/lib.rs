mod error;

use std::collections::HashMap;
use std::fs::{ File, self };
use std::io::BufWriter;
use failure::Error;
use std::path::{PathBuf};
use std::result;
use serde::{Deserialize, Serialize};
use crate::error::KvsError;

pub type Result<T> = result::Result<T, KvsError>;

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
    writer: File
}

// impl Default for KvStore {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl KvStore {
    // /// Creates a new `KvStore`
    // pub fn new() -> KvStore {
    //     KvStore {
    //         map: HashMap::new(),
    //         writer: file
    //     }
    // }

    /// Inserts the given value for the given key
    ///
    /// If the key already exists, the previous value will be replaced.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let tx = Transaction::Set { key: key.clone(), value: value.clone() };
        serde_json::to_writer(&self.writer, &tx)?;
        self.map.insert(key, value);
        Ok(())
    }

    /// Gets the string value for a given key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
    }

    /// Removes the given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        if let Some(value) = self.map.remove(&key) {
            return Ok(())
        }
        Err(KvsError::KeyNotFound)
    }

    /// Opens a KV Store from disk
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        fs::create_dir_all(&path)?;
        let log_file = path.join(format!("{}.log", 0));
        let file = File::create(&log_file).expect(&*format!("Unable to create file"));

        Ok(KvStore {
            map: HashMap::new(),
            writer: file
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Transaction {
    Set { key: String, value: String},
    Remove { key: String, value: String},
}
