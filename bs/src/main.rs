mod args;
use structopt::StructOpt;

fn main() {
    let opt = args::Opt::from_args();
    println!("{:?}", opt);
}
