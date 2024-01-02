//! Contains the entire global state of the application

use std::{collections::HashSet, fmt::Display, net::IpAddr};

use crossbeam_channel::Receiver;
use parking_lot::RwLock;
use pnet::{datalink::interfaces, util::MacAddr};
use tui_input::Input;

use crate::net_utils;

/// This enum represents the possible states of the text input box
pub(crate) enum InputMode {
    /// Unselected
    Normal,
    /// Editing
    Editing,
}

/// Whether we have a MAC address or IP address for a host
#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct Host {
    mac_address: MacAddr,
    ip_address: Option<IpAddr>,
    domain_name: Option<String>,
    open_ports: Vec<u16>
}

impl Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display: String = self.mac_address.to_string();
        if let Some(ip) = self.ip_address {
            display = format!("{ip}");
        }
        if let Some(domain) = &self.domain_name {
            display = domain.clone();
        }
        write!(f, "{display}")
    }
}

impl Host {
    fn new(mac_address: MacAddr) -> Self {
        Self {
            mac_address,
            ip_address: None,
            domain_name: None,
            open_ports: Vec::new()
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
    pub(crate) hosts: RwLock<HashSet<Host>>,
    /// Networking interface to use
    pub(crate) interface_name: String,
    /// If passive listening is enabled
    pub(crate) listening: bool,
    /// Thread receiver for listener thread
    pub(crate) listen_thread_rx: Option<Receiver<MacAddr>>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: Input::default(),
            input_mode: InputMode::Editing,
            messages: Vec::new(),
            last_error: None,
            hosts: RwLock::new(HashSet::new()),
            interface_name: net_utils::interface::find_default_interface().name,
            listening: false,
            listen_thread_rx: None,
        }
    }
}

impl App {
    /// Add a new mac address host if it doesn't already exist.
    pub(crate) fn new_mac(&self, new: MacAddr) {
        let mut write_handle = self.hosts.write();
        write_handle.insert(Host::new(new));
    }

    /// Get all currently stored hosts
    pub(crate) fn get_hosts(&self) -> HashSet<Host> {
        let read_handle = self.hosts.read();
        read_handle.clone()
    }
}
