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

fn main() {
    let binswapDir = "~/.binswap/"
    let args = Cli::from_args();
    println!("{:?}", args);
}
