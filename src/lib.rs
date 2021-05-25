//! An ERCP CLI builder.

use std::str::FromStr;

use ercp_basic::command::component;

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
