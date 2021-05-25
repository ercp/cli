#![deny(unsafe_code)]

use std::process;

use chrono::Local;
use structopt::StructOpt;

use ercp_cli::Component;
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

    /// Resets the device
    Reset,

    /// Gets the protocol version
    Protocol,

    /// Gets the version of a component
    Version { component: Component },

    /// Gets the maximum accepted value length
    MaxLength,

    /// Gets the device description
    Description,

    /// Sends a custom command
    Command {
        command: String,
        value: Option<String>,
    },

    /// Waits for and prints logs sent by the device
    Log,
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

        Command::Reset => {
            device.reset().ok();
        }

        Command::Protocol => match device.protocol() {
            Ok(version) => {
                println!(
                    "Protocol: ERCB Basic {}.{}.{}",
                    version.major, version.minor, version.patch
                )
            }
            Err(_) => eprintln!("An error has occured"),
        },

        Command::Version { component } => match device.version(component) {
            Ok(version) => println!("{}", version),
            Err(_) => eprintln!("An error has occured"),
        },

        Command::MaxLength => match device.max_length() {
            Ok(max_length) => println!("Max length = {}", max_length),
            Err(_) => eprintln!("An error has occured"),
        },

        Command::Description => match device.description() {
            Ok(description) => println!("{}", description),
            Err(_) => eprintln!("An error has occured"),
        },

        Command::Command { command, value } => {
            let command = u8::from_str_radix(&command, 16)?;
            let value = match value {
                Some(value) => Vec::<u8>::from_hex(&value)?,
                None => vec![],
            };

            match device.command(command, &value) {
                Ok(reply) => {
                    dbg!(reply);
                }

                Err(_) => eprintln!("An error has occured"),
            }
        }

        Command::Log => {
            println!(
                "{} Starting log session (type ^C to quit)",
                Local::now().format("%H:%M:%S%.3f")
            );

            loop {
                match device.wait_for_log() {
                    Ok(message) => {
                        let ts = Local::now();
                        println!("{} {}", ts.format("%H:%M:%S%.3f"), message);
                    }
                    Err(_) => eprintln!("An error has occured"),
                };
            }
        }
    }

    Ok(())
}
