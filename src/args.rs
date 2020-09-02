use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// binswap allows you to quickly swap out different binary versions
pub struct BinSwapOpts {
    // Default behaviour is to Swap
    /// Alias of the binary to swap to
    #[structopt(requires("version"))]
    pub name: Option<String>,
    /// Version of the binary to swap to
    #[structopt(requires("name"))]
    pub version: Option<String>,

    #[structopt(subcommand)]
    pub cmd: Option<SubCommand>,
    /// binswap config location, defaults to ~/.config/binswap.
    #[structopt(parse(from_os_str), env = "BINSWAP_CONFIG")]
    pub location: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    /// Initialise binswap
    Init {},
    /// Swap to a different version of the given binary
    Swap {
        name: String,
        /// Version of the binary to swap to
        version: String,
    },
    /// Add a binary to binswap, so that you then can swap to it
    Add {
        /// Alias of the binary to add
        name: String,
        /// Version of the binary to add
        version: String,
        /// Path to the binary to add
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
    /// Remove a binary from binswap
    Remove {
        /// Alias of the binary to add
        name: String,
        /// Version of the binary to add
        version: String,
    },
}
