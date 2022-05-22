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

//! ERCP CLI built-in options and commands.

use std::str::FromStr;

use ercp_device::command::component;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Protocol {
    /// Use ERCP Basic
    #[structopt(long, short)]
    pub basic: bool,
}

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
