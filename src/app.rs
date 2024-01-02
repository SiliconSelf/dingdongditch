//! This module contains all the structures and functionality related to
//! application state.
//!
//! The main purpose of this module is the [App] struct. It manages everything
//! from UI state to detected hosts.

use std::collections::HashSet;

use once_cell::sync::Lazy;
use parking_lot::RwLock;

use crate::net::Host;

/// Static value that acts as a thread-safe single source of truth for the
/// application state
pub(crate) static APP_STATE: Lazy<RwLock<App>> =
    Lazy::new(|| RwLock::new(App::default()));

/// Global application state
#[derive(Default)]
pub(crate) struct App {
    detected_hosts: HashSet<Host>,
}

// impl Default for App {
//     fn default() -> Self {
//         Self {
//             detected_hosts: HashSet::new()
//         }
//     }
// }

impl App {
    fn add_host(&mut self, new_host: Host) {
        self.detected_hosts.insert(new_host);
    }
}