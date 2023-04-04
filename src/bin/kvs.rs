extern crate exitcode;

use serde::{Deserialize, Serialize};
use clap::{Args, Parser, Subcommand};
use kvs::Result;

fn main() -> Result<()> {
    let args: KvArgs = KvArgs::parse();
    // println!("{:?}", args);

    match args.operation {
        Operation::Get(_) => {
            eprint!("unimplemented");
            std::process::exit(exitcode::CONFIG);
        }
        Operation::Set(cmd) => {
            // eprint!("unimplemented");
            // println!("The SetCommand is {:?}", cmd);
            let json = serde_json::to_string(&cmd);
            // println!("The json is {}", json.unwrap());
            std::process::exit(exitcode::OK);
        }
        Operation::Remove(_) => {
            eprint!("unimplemented");
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
