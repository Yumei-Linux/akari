mod utils {
    pub mod args;
}

use clap::Parser;
use crate::utils::args::Args;

fn main() {
    let args = Args::parse();

    println!("yumei root is at -> {}", args.yumei_root);
}
