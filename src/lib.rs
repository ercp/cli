//! An ERCP CLI builder.

pub mod opts;

mod router;

pub use router::{DefaultRouter, Router};

use std::process;

use ercp_device::Device;
use structopt::StructOpt;

use opts::{BuiltinCommand, Connection, Protocol};

/// The default ERCP CLI.
pub struct Cli {
    opts: Opts,
    router: DefaultRouter,
}

/// A command line tool for communicating with ERCP devices
#[derive(Debug, StructOpt)]
#[structopt(author = "Jean-Philippe Cugnet <jean-philippe@cugnet.eu>")]
pub struct Opts {
    #[structopt(flatten)]
    protocol: Protocol,

    #[structopt(flatten)]
    connection: Connection,

    #[structopt(subcommand)]
    command: BuiltinCommand,
}

impl Cli {
    pub fn new(opts: Opts) -> Self {
        let device = Device::new(&opts.connection.port);
        let router = DefaultRouter::new(device);

        Self { opts, router }
    }

    pub fn run(&mut self) {
        if !self.opts.protocol.basic {
            eprintln!("You must select a protocol.");
            process::exit(1);
        }

        self.router.route(&self.opts.command);
    }
}
