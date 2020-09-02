mod args;
mod commands;
use args::{BinSwapOpts, SubCommand};
use commands::BinSwap;
use dirs::home_dir;
use semver::Version;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

pub const ACTIVE: &str = "active";
pub const INACTIVE: &str = "inactive";
pub const BINSWAP_DEFAULT_LOCATION: &str = "binswap";
pub const BINSWAP_CONFIG_DIR_NAME: &str = "binswap";

fn main() {
    let opt = BinSwapOpts::from_args();
    run(opt)
}

fn get_config_location(input_location: Option<PathBuf>) -> PathBuf {
    let mut config_location = match input_location {
        Some(path) => path,
        None => home_dir().expect("No directory specified and can't get home directory!"),
    };
    config_location.push(BINSWAP_CONFIG_DIR_NAME);

    config_location
}

fn run(opt: BinSwapOpts) {
    let bs = BinSwap {
        config_location: get_config_location(opt.location),
        active_location_relative: PathBuf::from(ACTIVE),
        inactive_location_relative: PathBuf::from(INACTIVE),
    };

    let command = match opt.cmd {
        Some(cmd) => cmd,
        None => SubCommand::Swap {
            // Default to swap
            name: opt.name.unwrap(),
            version: opt.version.unwrap(),
        },
    };

    let res = match command {
        SubCommand::Init {} => bs.init(),
        SubCommand::Swap { name, version } => bs.swap(&name, &version),
        SubCommand::Add {
            name,
            version,
            path,
        } => bs.add(&name, &version, &path),
        SubCommand::Remove { name, version } => bs.remove(&name, &version),
    };

    res.unwrap();
}
