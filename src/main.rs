mod args;
mod commands;
use args::{BinSwapOpts, SubCommand};
use commands::BinSwap;
use dirs::home_dir;
use std::path::PathBuf;
use structopt::StructOpt;

pub const ACTIVE: &str = "active";
pub const ARCHIVE: &str = "archive";
pub const PROFILE: &str = ".profile";
pub const BINSWAP_DEFAULT_LOCATION: &str = ".config";
pub const BINSWAP_CONFIG_DIR_NAME: &str = "binswap";

fn main() {
    let opt = BinSwapOpts::from_args();
    run(opt)
}

fn get_config_location(input_location: Option<PathBuf>) -> PathBuf {
    let mut config_location = match input_location {
        Some(path) => path,
        None => home_dir()
            .expect("No directory specified and can't get home directory!")
            .join(&BINSWAP_DEFAULT_LOCATION),
    };
    config_location.push(&BINSWAP_CONFIG_DIR_NAME);

    config_location
}

fn run(opt: BinSwapOpts) {
    let bs = BinSwap {
        profile_location: home_dir().unwrap().join(PROFILE),
        config_location: get_config_location(opt.location),
        active_location_relative: PathBuf::from(ACTIVE),
        archive_location_relative: PathBuf::from(ARCHIVE),
    };

    let command = match opt.cmd {
        // If user has specified a command, do that
        Some(cmd) => cmd,
        // No command specified, seek out default behaviour
        None => {
            match opt.path {
                // If path specified, add that version
                Some(path) => SubCommand::Add {
                    name: opt.name.unwrap(),
                    version: opt.version.unwrap(),
                    path,
                },
                None => match opt.version {
                    // If version specified, swap to that version
                    Some(version) => SubCommand::Swap {
                        name: opt.name.unwrap(), // StructOpts catches the no name case for now
                        version: version,
                    },
                    // If no version is specified, show the active version
                    None => match opt.name {
                        Some(name) => SubCommand::Active { name },
                        None => {
                            println!("Run `bs --help` for usage info.");
                            return;
                        }
                    },
                },
            }
        }
    };

    let res = match &command {
        SubCommand::Init {} => bs.init(),
        SubCommand::Swap { name, version } => bs.swap(&name, &version),
        SubCommand::List { name } => bs.list(&name),
        SubCommand::Active { name } => bs.active(&name),
        SubCommand::Add {
            name,
            version,
            path,
        } => bs.add(&name, &version, &path),
        SubCommand::Remove { name, version } => bs.remove(&name, &version),
    };

    match res {
        Ok(_) => (),
        Err(e) => println!("{:?} failed:\n{:?}", &command, &e),
    }
}
