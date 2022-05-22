// Copyright 2021-2022 Jean-Philippe Cugnet
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An ERCP CLI builder.

pub mod opts;

mod router;

pub use router::{DefaultRouter, Router};

use std::process;

use clap::Parser;
use ercp_device::Device;

use opts::{BuiltinCommand, Connection, Options, Protocol};

/// The default ERCP CLI.
pub struct Cli {
    opts: Opts,
    router: DefaultRouter,
}

/// A command line tool for communicating with ERCP devices
#[derive(Debug, Parser)]
#[clap(author, version)]
pub struct Opts {
    #[clap(flatten)]
    protocol: Protocol,

    #[clap(flatten)]
    connection: Connection,

    #[clap(flatten)]
    options: Options,

    #[clap(subcommand)]
    command: BuiltinCommand,
}

impl Cli {
    pub fn new(opts: Opts) -> Self {
        let device = Device::new(&opts.connection.port)
            .expect("Failed to open the port");

        let router = DefaultRouter::new(device);

        Self { opts, router }
    }

    pub fn run(&mut self) {
        if !self.opts.protocol.basic {
            eprintln!("You must select a protocol.");
            process::exit(1);
        }

        self.router
            .route(&self.opts.command, self.opts.options.timeout());
    }
}
