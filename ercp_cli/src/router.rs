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

use std::{fmt::Display, time::Duration};

use chrono::Local;
use colored::Colorize;
use ercp_device::{CommandError, Device};
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
    ) -> Result<(), CommandError> {
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

    fn ping(&mut self, timeout: Option<Duration>) -> Result<(), CommandError> {
        match self.device().ping(timeout)? {
            Ok(()) => println!("{}", "The device is alive.".green().bold()),
            Err(e) => print_error(e),
        }

        Ok(())
    }

    fn reset(&mut self, timeout: Option<Duration>) -> Result<(), CommandError> {
        self.device().reset(timeout)?.ok();
        Ok(())
    }

    fn protocol(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<(), CommandError> {
        match self.device().protocol(timeout)? {
            Ok(version) => {
                println!(
                    "Protocol: ERCB Basic {}.{}.{}",
                    version.major, version.minor, version.patch
                )
            }
            Err(e) => print_error(e),
        }

        Ok(())
    }

    fn version(
        &mut self,
        component: &Component,
        timeout: Option<Duration>,
    ) -> Result<(), CommandError> {
        match self.device().version(component.into(), timeout)? {
            Ok(version) => println!("{version}"),
            Err(e) => print_error(e),
        }

        Ok(())
    }

    fn max_length(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<(), CommandError> {
        match self.device().max_length(timeout)? {
            Ok(max_length) => println!("Max length = {max_length}"),
            Err(e) => print_error(e),
        }

        Ok(())
    }

    fn description(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<(), CommandError> {
        match self.device().description(timeout)? {
            Ok(description) => println!("{description}"),
            Err(e) => print_error(e),
        }

        Ok(())
    }

    fn command(
        &mut self,
        command: &str,
        value: Option<&str>,
        timeout: Option<Duration>,
    ) -> Result<(), CommandError> {
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
            Err(e) => print_error(e),
        }

        Ok(())
    }

    fn log(&mut self) -> ! {
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
                Err(e) => print_error(e),
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
        if let Err(error) = self.builtin_commands(command, timeout) {
            let message = format!("Command error: {error}.").red().bold();
            eprintln!("{message}");
        }
    }
}

impl DefaultRouter {
    pub fn new(device: Device) -> Self {
        Self { device }
    }
}

/// Prints an error in red.
fn print_error<E: Display>(error: E) {
    let message = format!("Error: {error}.").red().bold();
    eprintln!("{message}");
}
