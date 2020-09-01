mod args;
use args::{BinSwapOpts, SubCommand};
use semver::Version;
use structopt::StructOpt;

#[derive(Debug)]
pub struct BinVer {
    name: String,
    ver: Version,
}

fn main() {
    let opt = BinSwapOpts::from_args();
    run(opt)
}

fn run(opt: BinSwapOpts) {
    let command = match opt.cmd {
        Some(cmd) => cmd,
        None => SubCommand::Swap {
            name: opt.name.unwrap(),
            version: opt.version.unwrap(),
        },
    };

    match command {
        SubCommand::Swap { name, version } => {}
        SubCommand::Add {
            name,
            version,
            path,
        } => {}
        SubCommand::Remove { name, version } => {}
    }
}
