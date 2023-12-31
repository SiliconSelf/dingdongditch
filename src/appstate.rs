//! Contains the entire global state of the application

use std::{fmt::Display, net::IpAddr, sync::mpsc::Receiver};

use parking_lot::RwLock;
use pnet::{util::MacAddr, datalink::interfaces};
use tui_input::Input;

/// This enum represents the possible states of the text input box
pub(crate) enum InputMode {
    /// Unselected
    Normal,
    /// Editing
    Editing,
}

/// Whether we have a MAC address or IP address for a host
#[derive(Clone)]
pub(crate) enum Host {
    /// We only have a MAC address
    Mac(MacAddr),
    /// We have discovered an IP
    Ip(IpAddr),
}

impl Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Host::Mac(a) => {
                write!(f, "{a}")
            }
            Host::Ip(a) => write!(f, "{a}"),
        }
    }
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
    pub(crate) hosts: RwLock<Vec<Host>>,
    /// Networking interface to use
    pub(crate) interface_name: String,
    /// If passive listening is enabled
    pub(crate) listening: bool,
    /// Thread receiver for listener thread
    pub(crate) listen_thread_rx: Option<Receiver<MacAddr>>,
}

impl Default for App {
    fn default() -> App {
        let interfaces = interfaces();
        let Some(first_interface) = interfaces.first() else { panic!("No interfaces are connected") };
        App {
            input: Input::default(),
            input_mode: InputMode::Editing,
            messages: Vec::new(),
            last_error: None,
            hosts: RwLock::new(Vec::new()),
            interface_name: first_interface.name.clone(),
            listening: false,
            listen_thread_rx: None,
        }
    }
}

impl App {
    /// Add a new mac address host if it doesn't already exist.
    pub(crate) fn new_mac(&self, new: MacAddr) {
        let mut write_handle = self.hosts.write();
        write_handle.push(Host::Mac(new));
    }

    /// Associate an IP with a MAC
    pub(crate) fn upgrade_to_ip(&self, mac: MacAddr, ip: IpAddr) {
        todo!();
    }

    /// Get all currently stored hosts
    pub(crate) fn get_hosts(&self) -> Vec<Host> {
        let read_handle = self.hosts.read();
        read_handle.clone()
    }
}
