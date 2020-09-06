mod args;
mod commands;
mod evm;
use args::{EvmOpts, SubCommand};
use commands::EvmConfig;
use dirs::home_dir;
use std::path::PathBuf;
use structopt::StructOpt;

pub const ACTIVE: &str = "active";
pub const ARCHIVE: &str = "archive";
pub const PROFILE: &str = ".profile";
pub const EVM_DEFAULT_LOCATION: &str = ".config";
pub const EVM_CONFIG_DIR_NAME: &str = "evm";

fn main() {
    let opt = EvmOpts::from_args();
    run(opt)
}

fn get_config_location(input_location: Option<PathBuf>) -> PathBuf {
    let mut config_location = match input_location {
        Some(path) => path,
        None => home_dir()
            .expect("No directory specified and can't get home directory!")
            .join(&EVM_DEFAULT_LOCATION),
    };
    config_location.push(&EVM_CONFIG_DIR_NAME);

    config_location
}

fn run(opt: EvmOpts) {
    let configuration = EvmConfig {
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
                            println!("Run `evm --help` for usage info.");
                            return;
                        }
                    },
                },
            }
        }
    };

    let res = match &command {
        SubCommand::Init {} => configuration.init(),
        SubCommand::Swap { name, version } => configuration.swap(&name, &version),
        SubCommand::List { name } => configuration.list(&name),
        SubCommand::Active { name } => configuration.active(&name),
        SubCommand::Add {
            name,
            version,
            path,
        } => configuration.add(&name, &version, &path),
        SubCommand::Remove { name, version } => configuration.remove(&name, &version),
    };

    match res {
        Ok(_) => (),
        Err(e) => println!("{:?} failed:\n{:?}", &command, &e),
    }
}
