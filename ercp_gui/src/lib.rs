mod connector;

use gtk::{
    prelude::*,
    Orientation::{Horizontal, Vertical},
};
use relm::{connect, Relm, Widget};
use relm_derive::{widget, Msg};

use ercp_device::{command::component, Device};
use hex::FromHex;

use connector::Connector;

pub struct Model {
    relm: Relm<Win>,
    // connector:
    // port: String,
    device: Option<Device>,
    // connection_status: String,
    description: String,
    firmware_version: String,
    ercp_library: String,
    command: String,
    value: String,
    reply: String,
}

#[derive(Msg)]
pub enum Msg {
    UpdateCommand(String),
    UpdateValue(String),
    Connect,
    Disconnect,
    SendCommand,
    Quit,
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
            device: None,
            description: String::from("N/A"),
            firmware_version: String::from("N/A"),
            ercp_library: String::from("N/A"),
            command: String::new(),
            value: String::new(),
            reply: String::new(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::UpdateCommand(command) => self.model.command = command,
            Msg::UpdateValue(value) => self.model.value = value,

            Msg::Connect => {
                println!("Connect from the main box");
                if let Some(device) = &mut self.model.device {
                    match device.description() {
                        Ok(description) => {
                            self.model.description = description;
                        }

                        Err(_) => {
                            self.model.description = String::from("Error :(");
                        }
                    }

                    match device.version(component::FIRMWARE) {
                        Ok(version) => {
                            self.model.firmware_version = version;
                        }

                        Err(_) => {
                            self.model.firmware_version =
                                String::from("Error :(");
                        }
                    }

                    match device.version(component::ERCP_LIBRARY) {
                        Ok(version) => self.model.ercp_library = version,

                        Err(_) => {
                            self.model.ercp_library = String::from("Error :(");
                        }
                    }
                }
            }

            Msg::Disconnect => {
                self.model.description = String::from("N/A");
                self.model.firmware_version = String::from("N/A");
                self.model.ercp_library = String::from("N/A");
                self.model.reply = String::new();
            }

            Msg::SendCommand => {
                fn parse(
                    command: &str,
                    value: &str,
                ) -> Result<(u8, Vec<u8>), Box<dyn std::error::Error>>
                {
                    let command = u8::from_str_radix(command, 16)?;
                    let value = Vec::<u8>::from_hex(value)?;
                    Ok((command, value))
                }

                if let Some(device) = &mut self.model.device {
                    match parse(&self.model.command, &self.model.value) {
                        Ok((command, value)) => {
                            match device.command(command, &value) {
                                Ok(reply) => {
                                    let command = reply.command();
                                    let value = reply.value();
                                    self.model.reply = format!(
                                        "{:02X?} {:02X?}",
                                        command, value
                                    );

                                    device.reset_ercp_state();
                                }

                                Err(_) => {
                                    self.model.reply = String::from("Error");
                                }
                            }
                        }

                        Err(_) => {
                            self.model.reply = String::from("Parse error");
                        }
                    }
                }
            }

            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                spacing: 20,

                // Connector {
                //     // TODO: Make this work.
                //     Connect => Msg::Connect,
                // },

                gtk::Box {
                    orientation: Vertical,

                    gtk::Box {
                        orientation: Horizontal,
                        gtk::Label { text: "Description: " },
                        gtk::Label { text: &self.model.description },
                    },

                    gtk::Box {
                        orientation: Horizontal,
                        gtk::Label { text: "Firmware version: " },
                        gtk::Label { text: &self.model.firmware_version },
                    },

                    gtk::Box {
                        orientation: Horizontal,
                        gtk::Label { text: "ERCP library: " },
                        gtk::Label { text: &self.model.ercp_library },
                    },
                },

                gtk::Box {
                    orientation: Horizontal,
                    homogeneous: true,
                    spacing: 10,

                    gtk::Box {
                        orientation: Horizontal,
                        homogeneous: true,

                        gtk::Entry {
                            max_length: 2,
                            changed(entry) => {
                                let command = entry.get_text().to_string();
                                Msg::UpdateCommand(command)
                            },
                        },

                        gtk::Entry {
                            changed(entry) => {
                                let value = entry.get_text().to_string();
                                Msg::UpdateValue(value)
                            },
                        },

                        gtk::Button {
                            label: "Send command",
                            clicked => Msg::SendCommand,
                        },
                    },

                    gtk::Box {
                        orientation: Horizontal,
                        gtk::Label { text: "Reply: " },
                        gtk::Label { text: &self.model.reply, },
                    },
                },
            },

            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        },
    }
}
