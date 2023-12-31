//! Contains the entire global state of the application

use std::net::IpAddr;

use tui_input::Input;

/// This enum represents the possible states of the text input box
pub(crate) enum InputMode {
    /// Unselected
    Normal,
    /// Editing
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
    /// Last error from a bad command
    pub(crate) last_error: Option<String>,
    /// Detected hosts
    pub(crate) hosts: Vec<IpAddr>,
    /// Networking interface to use
    pub(crate) interface_name: Option<String>,
    /// If passive listening is enabled
    pub(crate) listening: bool
}

impl Default for App {
    fn default() -> App {
        App {
            input: Input::default(),
            input_mode: InputMode::Editing,
            messages: Vec::new(),
            last_error: None,
            hosts: Vec::new(),
            interface_name: None,
            listening: false
        }
    }
}
