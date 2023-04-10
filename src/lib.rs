mod error;

use std::collections::HashMap;
use std::fs::{ File, self, OpenOptions };
use std::io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
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
    // map: HashMap<String, String>,
    map: HashMap<String, LogSection>,
    writer: TrackingBufWriter<File>,
    reader: BufReader<File>,
}

impl KvStore {

    /// Inserts the given file position for the given key
    ///
    /// If the key already exists, the previous position will be replaced.
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let pos_start = self.writer.pos;
        let command = Command::Set { key: key.clone(), value: value.clone() };
        serde_json::to_writer(&mut self.writer, &command)?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()?;
        self.map.insert(key, (pos_start, self.writer.pos).into());
        Ok(())
    }

    /// Gets the string value for a given key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(log_section) = self.map.get(&key) {
            println!("Found LogSection: {:?}", log_section);
            self.reader.seek(SeekFrom::Start(log_section.start))?;
            let mut buffer = vec![0; log_section.length as usize];
            self.reader.read_exact(&mut buffer)?;
            let command: Command = serde_json::from_slice(&buffer)?;
            return match command {
                Command::Set { value, .. } => {
                    println!("There is a set command here with value {}", value);
                    Ok(Some(value))
                }
                Command::Remove { .. } => {
                    Ok(None)
                }
            }
        }
        Ok(None)
    }

    /// Removes the given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        // println!("<<< Removing {} >>>", key);
        if let Some(value) = self.map.remove(&key) {
            // println!("<<< Removed {} >>>", value);
            let pos_start = self.writer.pos;
            let command = Command::Remove { key: key.clone() };
            serde_json::to_writer(&mut self.writer, &command)?;
            self.writer.write_all(b"\n")?;
            self.writer.flush()?;
            self.map.remove(&key);
            return Ok(())
        }
        Err(KvsError::KeyNotFound)
    }

    /// Opens a KV Store from disk
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        fs::create_dir_all(&path)?;
        let log_file = path.join(format!("{}.log", 0));

        let writer = TrackingBufWriter::new(
            OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&log_file)?)?;

        let reader = BufReader::new(OpenOptions::new()
            .read(true)
            .open(&log_file)?);

        let mut store = KvStore {
            map: HashMap::new(),
            writer,
            reader,
        };

        // KvStore::load(&mut store)?;

        Ok(store)
    }

    /// Reads the log file and populates the in-memory map
    /// Need to use read_line here as reader.lines() takes ownership which isn't very useful as it's on the struct
    pub fn load(store: &mut KvStore) -> Result<()>{
        // println!("Loading from logfile");
        // todo!("Will need to use tracked reader here");
        // let mut line = String::new();
        // while store.reader.read_line(&mut line)? > 0 {
        //     let command: Command = serde_json::from_str(&line)?;
        //     match command {
        //         Command::Set { key, value } => {
        //             // println!("Found SET command with key: {} and value: {}", key, value);
        //             // store.map.insert(key, value);
        //             store.map.insert(key, value);
        //         },
        //         Command::Remove { key } => {
        //             // println!("Found RM command with key: {} ", key);
        //             store.map.remove(&key);
        //         }
        //     }
        //     line.clear();
        // }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Command {
    Set { key: String, value: String},
    Remove { key: String },
}

pub struct TrackingBufWriter<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}

impl<W: Write + Seek> TrackingBufWriter<W> {
    fn new(mut inner: W) -> Result<Self> {
        println!("<<< Creating new writer >>>");
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(TrackingBufWriter { writer: BufWriter::new(inner), pos })
    }
}

impl<W: Write + Seek> Write for TrackingBufWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // println!("<<< Writing to writer >>>");
        // println!("<<< Current pos: {} >>>", self.pos);
        let bytes_written = self.writer.write(buf)?;
        // println!("<<< Bytes written: {} >>>", bytes_written);
        self.pos += bytes_written as u64;
        // println!("<<< Current pos: {} >>>", self.pos);
        Ok(bytes_written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write + Seek> Seek for TrackingBufWriter<W> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let pos = self.writer.seek(pos)?;
        Ok(pos)
    }
}

#[derive(Debug)]
struct LogSection {
    start: u64,
    length: u64,
}

impl From<(u64, u64)> for LogSection {
    fn from((start, end): (u64, u64)) -> Self {
        LogSection { start, length: end - start}
    }
}
