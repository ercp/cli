//! ERCP Device handling.

pub use ercp_basic::{command, error::CommandError, Command, Error};

use std::time::Duration;

use ercp_basic::{
    ack, adapter::SerialPortAdapter, command::LOG, DefaultRouter, ErcpBasic,
    Version,
};

/// An ERCP device.
pub struct Device {
    ercp: ErcpBasic<SerialPortAdapter, DefaultRouter, 255>,
}

impl Device {
    /// Creates a new device.
    pub fn new(port: &str) -> Self {
        let port = serialport::new(port, 115_200)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port");

        Self {
            ercp: ErcpBasic::new(SerialPortAdapter::new(port), DefaultRouter),
        }
    }

    /// Pings the device.
    pub fn ping(&mut self) -> Result<(), Error> {
        self.ercp.ping()
    }

    /// Resets the device.
    pub fn reset(&mut self) -> Result<(), Error> {
        self.ercp.reset()
    }

    /// Gets the protocol version implemented by the device.
    pub fn protocol(&mut self) -> Result<Version, Error> {
        self.ercp.protocol()
    }

    /// Gets the version of a component.
    pub fn version(&mut self, component: u8) -> Result<String, Error> {
        self.ercp.version_to_string(component)
    }

    /// Gets the maximum acceptable value length.
    pub fn max_length(&mut self) -> Result<u8, Error> {
        self.ercp.max_length()
    }

    /// Gets the device description.
    pub fn description(&mut self) -> Result<String, Error> {
        self.ercp.description_to_string()
    }

    /// Sends a command to the device.
    pub fn command(
        &mut self,
        command: u8,
        value: &[u8],
    ) -> Result<Command, Error> {
        let command = Command::new(command, value)?;
        self.ercp.command(command)
    }

    /// Waits for a log message from the device.
    pub fn wait_for_log(&mut self) -> Result<String, Error> {
        let notification = self.ercp.wait_for_command()?;

        if notification.command() == LOG {
            let message = String::from_utf8(notification.value().into())?;
            self.ercp.reset_state();
            self.ercp.notify(ack!()).ok();
            Ok(message)
        } else {
            Err(CommandError::UnexpectedReply.into())
        }
    }
}
