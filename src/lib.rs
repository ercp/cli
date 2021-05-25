//! An ERCP CLI builder.

use std::{str::FromStr, time::Duration};

use ercp_basic::{
    adapter::SerialPortAdapter, command::component, Command, DefaultRouter,
    ErcpBasic, Error, Version,
};

/// An ERCP device.
pub struct Device {
    ercp: ErcpBasic<SerialPortAdapter, DefaultRouter, 255>,
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

impl From<Component> for u8 {
    fn from(component: Component) -> Self {
        match component {
            Component::Firmware => component::FIRMWARE,
            Component::Ercp => component::ERCP_LIBRARY,
            Component::Other(value) => value,
        }
    }
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
    pub fn version(&mut self, component: Component) -> Result<String, Error> {
        self.ercp.version_to_string(component.into())
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
    ) -> Result<Command, ercp_basic::Error> {
        let command = Command::new(command, value)?;
        self.ercp.command(command)
    }
}
