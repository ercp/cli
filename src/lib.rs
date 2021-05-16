//! An ERCP CLI builder.

use std::time::Duration;

use ercp_basic::{
    adapter::SerialPortAdapter, Command, DefaultRouter, ErcpBasic,
};

/// An ERCP device.
pub struct Device {
    ercp: ErcpBasic<SerialPortAdapter, DefaultRouter, 255>,
}

impl Device {
    /// Creates a new device.
    pub fn new(port: String) -> Self {
        let port = serialport::new(port, 115_200)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port");

        Self {
            ercp: ErcpBasic::new(SerialPortAdapter::new(port), DefaultRouter),
        }
    }

    /// Pings the device.
    pub fn ping(&mut self) -> Result<(), ercp_basic::Error> {
        self.ercp.ping()
    }

    /// Sends a command to the device.
    pub fn command(
        &mut self,
        command: u8,
        value: &[u8],
    ) -> Result<Command, ercp_basic::Error> {
        let command = Command::new(command, value)?;
        self.ercp.command(command)
    }
}
