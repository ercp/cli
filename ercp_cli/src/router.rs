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

use std::time::Duration;

use chrono::Local;
use ercp_device::Device;
use hex::FromHex;

use crate::opts::{BuiltinCommand, Component};

pub trait Router {
    type Command;

    fn device(&mut self) -> &mut Device;

    fn route(&mut self, command: &Self::Command, timeout: Option<Duration>);

    fn builtin_commands(
        &mut self,
        command: &BuiltinCommand,
        timeout: Option<Duration>,
    ) {
        match command {
            BuiltinCommand::Ping => self.ping(timeout),
            BuiltinCommand::Reset => self.reset(timeout),
            BuiltinCommand::Protocol => self.protocol(timeout),
            BuiltinCommand::Version { component } => {
                self.version(component, timeout)
            }
            BuiltinCommand::MaxLength => self.max_length(timeout),
            BuiltinCommand::Description => self.description(timeout),
            BuiltinCommand::Log => self.log(),
            BuiltinCommand::Command { command, value } => {
                self.command(command, value.as_deref(), timeout)
            }
        }
    }

    fn ping(&mut self, timeout: Option<Duration>) {
        match self.device().ping(timeout) {
            Ok(Ok(())) => println!("Device: ACK"),
            _ => eprintln!("An error has occured"),
        }
    }

    fn reset(&mut self, timeout: Option<Duration>) {
        self.device().reset(timeout).ok();
    }

    fn protocol(&mut self, timeout: Option<Duration>) {
        match self.device().protocol(timeout) {
            Ok(Ok(version)) => {
                println!(
                    "Protocol: ERCB Basic {}.{}.{}",
                    version.major, version.minor, version.patch
                )
            }
            _ => eprintln!("An error has occured"),
        }
    }

    fn version(&mut self, component: &Component, timeout: Option<Duration>) {
        match self.device().version(component.into(), timeout) {
            Ok(Ok(version)) => println!("{}", version),
            _ => eprintln!("An error has occured"),
        }
    }

    fn max_length(&mut self, timeout: Option<Duration>) {
        match self.device().max_length(timeout) {
            Ok(Ok(max_length)) => println!("Max length = {}", max_length),
            _ => eprintln!("An error has occured"),
        }
    }

    fn description(&mut self, timeout: Option<Duration>) {
        match self.device().description(timeout) {
            Ok(Ok(description)) => println!("{}", description),
            _ => eprintln!("An error has occured"),
        }
    }

    fn command(
        &mut self,
        command: &str,
        value: Option<&str>,
        timeout: Option<Duration>,
    ) {
        // TODO: Handle errors.
        let command = u8::from_str_radix(command, 16).unwrap();
        let value = match value {
            Some(value) => Vec::<u8>::from_hex(value).unwrap(),
            None => vec![],
        };

        match self.device().command(command, &value, timeout) {
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
            match self.device().wait_for_log(None) {
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

    fn route(&mut self, command: &Self::Command, timeout: Option<Duration>) {
        self.builtin_commands(command, timeout)
    }
}

impl DefaultRouter {
    pub fn new(device: Device) -> Self {
        Self { device }
    }
}
