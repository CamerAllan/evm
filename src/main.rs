mod config;
use dirs;
use std::fs;
use std::path::Path;
use structopt::StructOpt;

// Command line arguments
#[derive(StructOpt, Debug)]
struct Cli {
    // The name of the binary, defaults to filename
    name: String,
    // The version of the binary
    version: String,
    // The path to the new binary we are adding
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    println!("{:?}", args);
    Ok(())
}
