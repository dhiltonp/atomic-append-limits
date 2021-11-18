use structopt::StructOpt;
use std::path::{Path, PathBuf};

#[derive(StructOpt)]
#[derive(Debug)]
struct Cli {
    #[structopt(short, long, default_value="/tmp/atomic-appends.txt")]
    file: PathBuf,

}

fn main() {
    let args = Cli::from_args();
    println!("{:#?}", args);
    println!("Hello, world!");
}
