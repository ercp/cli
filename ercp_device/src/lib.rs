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
    command, Command, DescriptionAsStringError, MaxLengthError, PingError,
    ProtocolError, ResetError, Version, VersionAsStringError,
};

pub use crate::error::{
    CommandError, CommandResult, CustomCommandError, LogNotificationError,
    ReceivedCommandError,
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

        port.clear(serialport::ClearBuffer::All).unwrap();

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
    pub fn ping(
        &mut self,
        timeout: Option<Duration>,
    ) -> CommandResult<(), PingError> {
        self.ercp.ping(timeout)
    }

    /// Resets the device.
    pub fn reset(
        &mut self,
        timeout: Option<Duration>,
    ) -> CommandResult<(), ResetError> {
        self.ercp.reset(timeout)
    }

    /// Gets the protocol version implemented by the device.
    pub fn protocol(
        &mut self,
        timeout: Option<Duration>,
    ) -> CommandResult<Version, ProtocolError> {
        self.ercp.protocol(timeout)
    }

    /// Gets the version of a component.
    pub fn version(
        &mut self,
        component: u8,
        timeout: Option<Duration>,
    ) -> CommandResult<String, VersionAsStringError> {
        self.ercp.version_as_string(component, timeout)
    }

    /// Gets the maximum acceptable value length.
    pub fn max_length(
        &mut self,
        timeout: Option<Duration>,
    ) -> CommandResult<u8, MaxLengthError> {
        self.ercp.max_length(timeout)
    }

    /// Gets the device description.
    pub fn description(
        &mut self,
        timeout: Option<Duration>,
    ) -> CommandResult<String, DescriptionAsStringError> {
        self.ercp.description_as_string(timeout)
    }

    /// Sends a command to the device.
    pub fn command(
        &mut self,
        code: u8,
        value: &[u8],
        timeout: Option<Duration>,
    ) -> Result<(u8, Vec<u8>), CustomCommandError> {
        self.ercp.command(|mut commander| {
            let command = Command::new(code, value)?;
            let reply = commander.transcieve(command, timeout)?;
            Ok((reply.code(), reply.value().into()))
        })
    }

    /// Waits for a log message from the device.
    pub fn wait_for_log(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<String, LogNotificationError> {
        let result = self.ercp.command(|mut commander| {
            let notification = commander.receive(timeout)?;

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
