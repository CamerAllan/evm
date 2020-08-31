use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// binswap allows you to quickly swap out different binary versions
pub struct Opt {

    /// Add a versioned binary
    #[structopt(short, long)]
    add: bool,
    
    /// The name of the binary
    name: String,

    /// The version of the binary
    version: String,

    /// The path to the binary we are adding
    #[structopt(parse(from_os_str), required_if("add", "true"), requires("add"))]
    path: Option<PathBuf>,
}
