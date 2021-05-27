//! ERCP CLI command router.

use chrono::Local;
use ercp_device::Device;
use hex::FromHex;

use crate::opts::{BuiltinCommand, Component};

pub trait Router {
    type Command;

    fn device(&mut self) -> &mut Device;

    fn route(&mut self, command: &Self::Command);

    fn builtin_commands(&mut self, command: &BuiltinCommand) {
        match command {
            BuiltinCommand::Ping => self.ping(),
            BuiltinCommand::Reset => self.reset(),
            BuiltinCommand::Protocol => self.protocol(),
            BuiltinCommand::Version { component } => self.version(component),
            BuiltinCommand::MaxLength => self.max_length(),
            BuiltinCommand::Description => self.description(),
            BuiltinCommand::Log => self.log(),
            BuiltinCommand::Command { command, value } => {
                self.command(command, value.as_deref())
            }
        }
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

/// The default command runner.
pub struct DefaultRouter {
    device: Device,
}

impl Router for DefaultRouter {
    type Command = BuiltinCommand;

    fn device(&mut self) -> &mut Device {
        &mut self.device
    }

    fn route(&mut self, command: &Self::Command) {
        self.builtin_commands(command)
    }
}

impl DefaultRouter {
    pub fn new(device: Device) -> Self {
        Self { device }
    }
}
