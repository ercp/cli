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
            Ok(Ok(())) => println!("Device: ACK"),
            _ => eprintln!("An error has occured"),
        }
    }

    fn reset(&mut self) {
        self.device().reset().ok();
    }

    fn protocol(&mut self) {
        match self.device().protocol() {
            Ok(Ok(version)) => {
                println!(
                    "Protocol: ERCB Basic {}.{}.{}",
                    version.major, version.minor, version.patch
                )
            }
            _ => eprintln!("An error has occured"),
        }
    }

    fn version(&mut self, component: &Component) {
        match self.device().version(component.into()) {
            Ok(Ok(version)) => println!("{}", version),
            _ => eprintln!("An error has occured"),
        }
    }

    fn max_length(&mut self) {
        match self.device().max_length() {
            Ok(Ok(max_length)) => println!("Max length = {}", max_length),
            _ => eprintln!("An error has occured"),
        }
    }

    fn description(&mut self) {
        match self.device().description() {
            Ok(Ok(description)) => println!("{}", description),
            _ => eprintln!("An error has occured"),
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
