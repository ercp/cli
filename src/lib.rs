//! An ERCP CLI builder.

use std::{process, str::FromStr};

use chrono::Local;
use ercp_basic::command::component;
use ercp_device::Device;
use hex::FromHex;
use structopt::StructOpt;

/// The default ERCP CLI.
pub struct DefaultCli {
    opts: DefaultOpts,
    runner: DefaultCommandRunner,
}

/// A command line tool for communicating with ERCP devices
#[derive(Debug, StructOpt)]
#[structopt(author = "Jean-Philippe Cugnet <jean-philippe@cugnet.eu>")]
pub struct DefaultOpts {
    #[structopt(flatten)]
    protocol: Protocol,

    #[structopt(flatten)]
    connection: Connection,

    #[structopt(subcommand)]
    command: BuiltinCommand,
}

/// Protocol options.
#[derive(Debug, StructOpt)]
pub struct Protocol {
    /// Use ERCP Basic
    #[structopt(long, short)]
    pub basic: bool,
}

/// Connection options.
#[derive(Debug, StructOpt)]
pub struct Connection {
    /// The serial port to use
    #[structopt(long, short)]
    pub port: String,
}

#[derive(Debug, StructOpt)]
pub enum BuiltinCommand {
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

/// A software component in the device.
#[derive(Debug)]
pub enum Component {
    Firmware,
    Ercp,
    Other(u8),
}

/// The default command runner.
struct DefaultCommandRunner {
    device: Device,
}

impl FromStr for Component {
    type Err = &'static str;

    fn from_str(component: &str) -> Result<Self, Self::Err> {
        match component {
            "firmware" => Ok(Component::Firmware),
            "fw" => Ok(Component::Firmware),
            "ercp" => Ok(Component::Ercp),
            _ => match u8::from_str_radix(component, 16) {
                Ok(value) => Ok(Component::Other(value)),
                Err(_) => Err("Could not parse a compoment"),
            },
        }
    }
}

impl From<&Component> for u8 {
    fn from(component: &Component) -> Self {
        match component {
            Component::Firmware => component::FIRMWARE,
            Component::Ercp => component::ERCP_LIBRARY,
            Component::Other(value) => *value,
        }
    }
}

impl DefaultCli {
    pub fn new(opts: DefaultOpts) -> Self {
        let device = Device::new(&opts.connection.port);
        let runner = DefaultCommandRunner::new(device);

        Self { opts, runner }
    }

    pub fn run(&mut self) {
        if !self.opts.protocol.basic {
            eprintln!("You must select a protocol.");
            process::exit(1);
        }

        self.handle_command();
    }

    fn handle_command(&mut self) {
        match &self.opts.command {
            BuiltinCommand::Ping => self.runner.ping(),
            BuiltinCommand::Reset => self.runner.reset(),
            BuiltinCommand::Protocol => self.runner.protocol(),
            BuiltinCommand::Version { component } => {
                self.runner.version(component)
            }
            BuiltinCommand::MaxLength => self.runner.max_length(),
            BuiltinCommand::Description => self.runner.description(),
            BuiltinCommand::Log => self.runner.log(),
            BuiltinCommand::Command { command, value } => {
                self.runner.command(command, value.as_deref())
            }
        };
    }
}

impl DefaultCommandRunner {
    pub fn new(device: Device) -> Self {
        Self { device }
    }

    fn device(&mut self) -> &mut Device {
        &mut self.device
    }

    fn ping(&mut self) {
        match self.device().ping() {
            Ok(()) => println!("Device: ACK"),
            Err(_) => eprintln!("An error has occured"),
        }
    }

    fn reset(&mut self) {
        self.device().reset().ok();
    }

    fn protocol(&mut self) {
        match self.device().protocol() {
            Ok(version) => {
                println!(
                    "Protocol: ERCB Basic {}.{}.{}",
                    version.major, version.minor, version.patch
                )
            }
            Err(_) => eprintln!("An error has occured"),
        }
    }

    fn version(&mut self, component: &Component) {
        match self.device().version(component.into()) {
            Ok(version) => println!("{}", version),
            Err(_) => eprintln!("An error has occured"),
        }
    }

    fn max_length(&mut self) {
        match self.device().max_length() {
            Ok(max_length) => println!("Max length = {}", max_length),
            Err(_) => eprintln!("An error has occured"),
        }
    }

    fn description(&mut self) {
        match self.device().description() {
            Ok(description) => println!("{}", description),
            Err(_) => eprintln!("An error has occured"),
        }
    }

    fn command(&mut self, command: &str, value: Option<&str>) {
        // TODO: Handle errors.
        let command = u8::from_str_radix(command, 16).unwrap();
        let value = match value {
            Some(value) => Vec::<u8>::from_hex(value).unwrap(),
            None => vec![],
        };

        match self.device().command(command, &value) {
            Ok(reply) => {
                dbg!(reply);
            }

            Err(_) => eprintln!("An error has occured"),
        }
    }

    fn log(&mut self) {
        println!(
            "{} Starting log session (type ^C to quit)",
            Local::now().format("%H:%M:%S%.3f")
        );

        loop {
            match self.device().wait_for_log() {
                Ok(message) => {
                    let ts = Local::now();
                    println!("{} {}", ts.format("%H:%M:%S%.3f"), message);
                }
                Err(_) => eprintln!("An error has occured"),
            };
        }
    }
}
