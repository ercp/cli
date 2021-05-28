#![deny(unsafe_code)]

use ercp_cli::{Cli, Opts};
use structopt::StructOpt;

fn main() {
    let opts = Opts::from_args();
    let mut cli = Cli::new(opts);
    cli.run()
}
