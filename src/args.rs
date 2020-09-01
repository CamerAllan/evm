use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// binswap allows you to quickly swap out different binary versions
pub struct BinSwapOpts {
    // Default behaviour is to Swap
    #[structopt(requires("version"))]
    pub name: Option<String>,
    #[structopt(requires("name"))]
    pub version: Option<String>,

    #[structopt(subcommand)]
    pub cmd: Option<SubCommand>,
}

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    Swap {
        name: String,
        version: String,
    },
    Add {
        name: String,
        version: String,
        #[structopt(parse(from_os_str))]
        path: PathBuf,
    },
    Remove {
        name: String,
        version: String,
    },
}
