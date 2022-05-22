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

//! ERCP Device handling.

mod error;

pub use ercp_basic::{
    command, Command, CommandError, DescriptionAsStringError, MaxLengthError,
    PingError, ProtocolError, ResetError, Version, VersionAsStringError,
};

pub use crate::error::{
    CommandResult, CustomCommandError, LogNotificationError,
};

use std::time::Duration;

use ercp_basic::{
    ack, adapter::SerialPortAdapter, command::LOG, timer::StdTimer,
    DefaultRouter, ErcpBasic,
};

/// An ERCP device.
pub struct Device {
    ercp: ErcpBasic<SerialPortAdapter, StdTimer, DefaultRouter>,
}

impl Device {
    /// Creates a new device.
    pub fn new(port: &str) -> Result<Self, serialport::Error> {
        let port = serialport::new(port, 115_200)
            .timeout(Duration::from_millis(10))
            .open()?;

        let device = Self {
            ercp: ErcpBasic::new(
                SerialPortAdapter::new(port),
                StdTimer,
                DefaultRouter,
            ),
        };

        Ok(device)
    }

    /// Pings the device.
    pub fn ping(&mut self) -> CommandResult<(), PingError> {
        self.ercp.ping(None)
    }

    /// Resets the device.
    pub fn reset(&mut self) -> CommandResult<(), ResetError> {
        self.ercp.reset(None)
    }

    /// Gets the protocol version implemented by the device.
    pub fn protocol(&mut self) -> CommandResult<Version, ProtocolError> {
        self.ercp.protocol(None)
    }

    /// Gets the version of a component.
    pub fn version(
        &mut self,
        component: u8,
    ) -> CommandResult<String, VersionAsStringError> {
        self.ercp.version_as_string(component, None)
    }

    /// Gets the maximum acceptable value length.
    pub fn max_length(&mut self) -> CommandResult<u8, MaxLengthError> {
        self.ercp.max_length(None)
    }

    /// Gets the device description.
    pub fn description(
        &mut self,
    ) -> CommandResult<String, DescriptionAsStringError> {
        self.ercp.description_as_string(None)
    }

    /// Sends a command to the device.
    pub fn command(
        &mut self,
        code: u8,
        value: &[u8],
    ) -> Result<(u8, Vec<u8>), CustomCommandError> {
        self.ercp.command(|mut commander| {
            let command = Command::new(code, value)?;
            let reply = commander.transcieve(command, None)?;
            Ok((reply.code(), reply.value().into()))
        })
    }

    /// Waits for a log message from the device.
    pub fn wait_for_log(&mut self) -> Result<String, LogNotificationError> {
        let result = self.ercp.command(|mut commander| {
            let notification = commander.receive(None)?;

            if notification.code() == LOG {
                let message = String::from_utf8(notification.value().into())?;
                Ok(message)
            } else {
                Err(LogNotificationError::UnexpectedFrame)
            }
        });

        if result.is_ok() {
            self.ercp.notify(ack!()).ok();
        }

        result
    }
}
