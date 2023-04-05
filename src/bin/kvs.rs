extern crate exitcode;

use std::env;
use serde::{Deserialize, Serialize};
use clap::{Args, Parser, Subcommand};
use kvs::{KvStore, Result};
use env::current_dir;

fn main() -> Result<()> {
    let args: KvArgs = KvArgs::parse();

    match args.operation {
        Operation::Get(cmd) => {
            let mut store = KvStore::open(current_dir()?)?;
            if let Some(value) = store.get(cmd.key)? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
            std::process::exit(exitcode::OK);
        }
        Operation::Set(cmd) => {
            let mut store = KvStore::open(current_dir()?)?;
            store.set(cmd.key, cmd.value)?;
            std::process::exit(exitcode::OK);
        }
        Operation::Remove(cmd) => {
            let mut store = KvStore::open(current_dir()?)?;
            if let Ok(_) = store.remove(cmd.key) {
                std::process::exit(exitcode::OK);
            }
            println!("Key not found");
            std::process::exit(exitcode::CONFIG);
        }
    }
}

/// Reads and Analyses Files
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct KvArgs {
    /// Operation to perform on KV
    #[clap(subcommand)]
    pub operation: Operation,
}

#[derive(Debug, Subcommand)]
pub enum Operation {
    /// Get a value by key
    Get(GetCliCommand),

    /// Set a value by key
    Set(SetCliCommand),

    /// Remove a value by key
    #[clap(name = "rm")]
    Remove(RemoveCliCommand),
}

#[derive(Args, Debug, Deserialize, Serialize)]
pub struct GetCliCommand {
    /// Name of key to get value for
    key: String,
}

#[derive(Args, Debug, Deserialize, Serialize)]
pub struct SetCliCommand {
    /// Name of key to get value for
    key: String,
    /// Value to set for key
    value: String,
}

#[derive(Args, Debug, Deserialize, Serialize)]
pub struct RemoveCliCommand {
    /// Name of key to remove value for
    key: String,
}
