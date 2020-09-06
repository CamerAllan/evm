use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// evm allows you to quickly swap out different binary versions
pub struct EvmOpts {
    /// Name of the binary
    #[structopt()]
    pub name: Option<String>,
    /// Version of the binary
    #[structopt(requires("name"))]
    pub version: Option<String>,
    /// Path to the binary to add
    #[structopt(requires("version"), parse(from_os_str))]
    pub path: Option<PathBuf>,

    #[structopt(subcommand)]
    pub cmd: Option<SubCommand>,
    /// evm config location, defaults to ~/.config/evm.
    #[structopt(parse(from_os_str), env = "EVM_CONFIG")]
    pub location: Option<PathBuf>,
}

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    /// Initialise evm
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
    /// Get active version of the given binary
    Active {
        /// Name of binary to view the active version of
        name: String,
    },
    /// Add a binary to evm, so that you then can swap to it
    Add {
        /// Name of the binary to add
        name: String,
        /// Version of the binary to add
        version: String,
        /// Path to the binary to add
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
    /// Remove a binary from evm
    Remove {
        /// Name of the binary to add
        name: String,
        /// Version of the binary to add
        version: Option<String>,
    },
}
