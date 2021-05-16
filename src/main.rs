#![deny(unsafe_code)]

use std::process;

use structopt::StructOpt;

use ercp_cli::Device;
use hex::FromHex;

/// A command line tool for communicating with ERCP devices
#[derive(Debug, StructOpt)]
#[structopt(author = "Jean-Philippe Cugnet <jean-philippe@cugnet.eu>")]
struct Cli {
    /// Use ERCP Basic
    #[structopt(long, short)]
    basic: bool,

    /// The serial port to use
    #[structopt(long, short)]
    port: String,

    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Tests communication with the device
    Ping,

    /// Sends a custom command
    Command { command: String, value: String },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::from_args();

    if !cli.basic {
        eprintln!("You must select a protocol.");
        process::exit(1);
    }

    let mut device = Device::new(cli.port);

    match cli.command {
        Command::Ping => match device.ping() {
            Ok(()) => println!("Device: ACK"),
            Err(_) => eprintln!("An error has occured"),
        },

        Command::Command { command, value } => {
            let command = u8::from_str_radix(&command, 16)?;
            let value = Vec::<u8>::from_hex(&value)?;

            match device.command(command, &value) {
                Ok(reply) => {
                    dbg!(reply);
                }

                Err(_) => eprintln!("An error has occured"),
            }
        }
    }

    Ok(())
}
