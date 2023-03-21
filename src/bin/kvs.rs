extern crate exitcode;

use std::io::stderr;
use clap::{Args, Parser, Subcommand, ValueEnum};

fn main() {
    let args: KvArgs = KvArgs::parse();
    println!("{:?}", args);

    match args.operation {
        Operation::Get(_) => {
            eprint!("unimplemented");
            std::process::exit(exitcode::CONFIG);
        }
        Operation::Set(_) => {}
        Operation::Remove(_) => {}
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
    Get(KeyOnlyCommand),

    /// Set a value by key
    Set(KeyAndValueCommand),

    /// Remove a value by key
    #[clap(name = "rm")]
    Remove(KeyOnlyCommand),
}

#[derive(Args, Debug)]
pub struct KeyOnlyCommand {
    /// Name of key to get value for
    key: String,
}

#[derive(Args, Debug)]
pub struct KeyAndValueCommand {
    /// Name of key to get value for
    key: String,
    /// Value to set for key
    value: String,
}