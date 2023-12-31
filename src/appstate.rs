use std::net::IpAddr;

use tui_input::Input;

pub(crate) enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
pub(crate) struct App {
    /// Current value of the input box
    pub(crate) input: Input,
    /// Current input mode
    pub(crate) input_mode: InputMode,
    /// History of recorded messages
    pub(crate) messages: Vec<String>,
    /// Detected hosts
    pub(crate) hosts: Vec<IpAddr>,
    /// Networking interface to use
    pub(crate) interface_name: Option<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: Input::default(),
            input_mode: InputMode::Editing,
            messages: Vec::new(),
            hosts: Vec::new(),
            interface_name: None,
        }
    }
}
