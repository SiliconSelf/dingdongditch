//! This module contains all the structures and functionality related to
//! application state.
//!
//! The main purpose of this module is the [App] struct. It manages everything
//! from UI state to detected hosts.

use std::collections::{HashSet, VecDeque};

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use pnet::datalink::interfaces;
use tui_input::Input;

use crate::net::{find_plausible_interface, interface_exists, Host};

/// Static value that acts as a thread-safe single source of truth for the
/// application state
pub(crate) static APP_STATE: Lazy<RwLock<App>> =
    Lazy::new(|| RwLock::new(App::default()));

/// What modes that UI input box can be in
#[derive(Default)]
pub(crate) enum InputMode {
    /// The text is being edited
    #[default]
    Editing,
    /// The textbox is deselected
    Normal,
}

/// Possible errors that can be returned from this function
pub(crate) enum Errors {
    /// No such interface exists
    NoSuchInterface,
}

/// Global application state
pub(crate) struct App {
    /// Hosts that have been detected by the program
    detected_hosts: HashSet<Host>,
    /// The last error output by the program to display in the UI
    last_error: Option<String>,
    /// The current text box mode
    input_mode: InputMode,
    /// The current input in the text box
    input: Input,
    /// The current queue of commands to process
    commands: VecDeque<String>,
    /// The currently selected interface
    interface_name: String,
    /// If the program is currently listening
    listening: bool,
}

impl Default for App {
    fn default() -> Self {
        let interface;
        if let Some(i) = find_plausible_interface() {
            interface = i;
        } else {
            interface = interfaces()[0].name.clone();
        }
        Self {
            detected_hosts: HashSet::new(),
            last_error: None,
            input_mode: InputMode::default(),
            input: Input::default(),
            commands: VecDeque::new(),
            interface_name: interface,
            listening: false,
        }
    }
}

impl App {
    /// Add a host to the internal `HashSet`
    pub(crate) fn add_host(&mut self, new_host: Host) {
        self.detected_hosts.insert(new_host);
    }

    /// Get a reference to the internal `HashSet`
    pub(crate) fn get_hosts(&self) -> &HashSet<Host> {
        &self.detected_hosts
    }

    /// Get a reference to the last error
    pub(crate) fn get_last_error(&self) -> &Option<String> {
        &self.last_error
    }

    /// Get a reference to the current input mode
    pub(crate) fn get_input_mode(&self) -> &InputMode {
        &self.input_mode
    }

    /// Get a mutable reference to the current input mode
    pub(crate) fn input_mode(&mut self, new_mode: InputMode) {
        self.input_mode = new_mode;
    }

    /// Get a reference to the current input
    pub(crate) fn get_input(&self) -> &Input {
        &self.input
    }

    /// Get a mutable reference to the input
    pub(crate) fn get_input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    /// Get a mutable reference to the queued commands
    pub(crate) fn get_commands_mut(&mut self) -> &mut VecDeque<String> {
        &mut self.commands
    }

    /// Push a command to the stack
    pub(crate) fn push_command(&mut self, command: &str) {
        self.commands.push_back(command.to_owned());
    }

    /// Set the last error to a new String or clear it with None
    pub(crate) fn last_error(&mut self, last_error: Option<String>) {
        self.last_error = last_error;
    }

    /// Get the current interface name
    pub(crate) fn get_interface_name(&self) -> &str {
        &self.interface_name
    }

    /// Change the current interface
    pub(crate) fn interface_name(
        &mut self,
        interface_name: &str,
    ) -> Result<(), Errors> {
        if interface_exists(interface_name) {
            self.interface_name = interface_name.to_owned();
            Ok(())
        } else {
            Err(Errors::NoSuchInterface)
        }
    }

    /// Return listening status
    pub(crate) fn get_listening(&self) -> bool {
        self.listening
    }

    /// Toggle listening
    pub(crate) fn toggle_listening(&mut self) {
        self.listening = !self.listening;
    }
}
