mod utils {
    pub mod args;
    pub mod exec;
}

mod core {
    pub mod base;
}

use clap::Parser;

use crate::utils::args::Args;
use crate::core::base::Base;

fn main() {
    let args = Args::parse();
    let base = Base::new(args.yumei_root);

    base.setup();
}
