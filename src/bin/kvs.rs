extern crate clap;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use kvs::kvstore::KvStore;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    set {
        key: String,
        val: String,
    },
    get {
        key: String,
    },
    rm {
        key: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut store = KvStore::open(&PathBuf::from("log.txt")).unwrap();
    // Your implementation here
    match &cli.command.unwrap() {
        Commands::get { key } => {
            let val = store.get(key.to_string());
            match val.unwrap(){
                Some(d) => println!("{}",d),
                None => println!("Key not found")
            }
        }
        Commands::rm { key } => {
            println!("{}", key);
            store.remove(key.to_string());
        }
        Commands::set { key, val } => {
            store.set(key.to_string(), val.to_string());
        }
    }
}
