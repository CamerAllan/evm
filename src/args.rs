use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// binswap allows you to quickly swap out different binary versions
pub struct BinSwapOpts {
    /// Alias of the binary
    #[structopt()]
    pub name: Option<String>,
    /// Version of the binary
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
        /// Name of binary to swap to
        name: String,
        /// Version of the binary to swap to
        version: String,
    },
    /// List all versions of a given binary
    List {
        /// Name of the binary to list versions of
        name: String,
    },
    /// Get current version of the given binary
    Current {
        /// Name of binary to view the current version of
        name: String,
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
        version: Option<String>,
    },
}
