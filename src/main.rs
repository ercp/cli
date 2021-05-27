#![deny(unsafe_code)]

use ercp_cli::{DefaultCli, DefaultOpts};
use structopt::StructOpt;

fn main() {
    let opts = DefaultOpts::from_args();
    let mut cli = DefaultCli::new(opts);
    cli.run()
}
