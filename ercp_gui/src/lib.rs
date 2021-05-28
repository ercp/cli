use gtk::{
    prelude::*,
    Orientation::{Horizontal, Vertical},
};
use relm::{connect, Relm, Widget};
use relm_derive::{widget, Msg};

use ercp_device::{command::component, Device};

pub struct Model {
    relm: Relm<Win>,
    port: String,
    device: Option<Device>,
    connection_status: String,
    description: String,
    firmware_version: String,
    ercp_library: String,
}

#[derive(Msg)]
pub enum Msg {
    UpdatePort(String),
    Connect,
    Disconnect,
    Quit,
}

#[widget]
impl Widget for Win {
    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            relm: relm.clone(),
            port: String::new(),
            device: None,
            connection_status: String::from("Disconnected."),
            description: String::from("N/A"),
            firmware_version: String::from("N/A"),
            ercp_library: String::from("N/A"),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::UpdatePort(port) => self.model.port = port,

            Msg::Connect => match Device::new(&self.model.port) {
                Ok(device) => {
                    self.model.device = Some(device);
                    self.model.connection_status =
                        format!("Connected to {}.", self.model.port);

                    self.widgets.connect_button.set_label("Disconnect");
                    connect!(
                        self.widgets.connect_button,
                        connect_clicked(_),
                        self.model.relm,
                        Msg::Disconnect
                    );

                    if let Some(device) = &mut self.model.device {
                        match device.description() {
                            Ok(description) => {
                                self.model.description = description;
                            }

                            Err(_) => {
                                self.model.description =
                                    String::from("Error :(");
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
                                self.model.ercp_library =
                                    String::from("Error :(");
                            }
                        }
                    }
                }

                Err(error) => {
                    self.model.connection_status =
                        format!("Error: {}.", error.to_string());
                }
            },

            Msg::Disconnect => {
                self.model.device = None;
                self.model.connection_status = String::from("Disconnected.");
                self.model.description = String::from("N/A");
                self.model.firmware_version = String::from("N/A");
                self.model.ercp_library = String::from("N/A");

                self.widgets.connect_button.set_label("Connect");
                connect!(
                    self.widgets.connect_button,
                    connect_clicked(_),
                    self.model.relm,
                    Msg::Connect
                );
            }

            Msg::Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,

                gtk::Entry {
                    placeholder_text: Some("TTY port path"),
                    changed(entry) => {
                        let port = entry.get_text().to_string();
                        Msg::UpdatePort(port)
                    },
                },

                gtk::Box {
                    orientation: Horizontal,
                    homogeneous: true,

                    #[name = "connect_button"]
                    gtk::Button {
                        label: "Connect",
                        clicked => Msg::Connect,
                    },

                    gtk::Label {
                        text: &self.model.connection_status,
                    },
                },

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
            },

            delete_event(_, _) => (Msg::Quit, Inhibit(false)),
        },
    }
}
