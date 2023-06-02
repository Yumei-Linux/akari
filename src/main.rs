mod utils {
    pub mod args;
    pub mod exec;
}

mod core {
    pub mod base;
    pub mod builder;
}

use clap::Parser;

use crate::utils::args::Args;
use crate::core::{base::Base, builder::Builder};

fn main() {
    let args = Args::parse();
    let base = Base::new(args.yumei_root);
    base.setup();

    let builder = Builder::from_base(base);
    builder.setup_builders();
    builder.run("pass1");
}
