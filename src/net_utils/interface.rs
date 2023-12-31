//! All hardware interface related functionality lives here

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use pnet::datalink::{dummy::interfaces, NetworkInterface};

use crate::appstate::App;

/// A collection of interfaces connected to the system
static INTERFACES: Lazy<RwLock<Vec<NetworkInterface>>> =
    Lazy::new(|| RwLock::new(interfaces()));

/// Change the interface the program is using
pub(crate) fn change_interface(app: &mut App, new_interface: String) {
    let read_handle = INTERFACES.read();
    // Check if the selected interface is real
    let interface_names =
        read_handle.iter().map(|x| &x.name).collect::<Vec<&String>>();
    if !interface_names.contains(&&new_interface) {
        app.last_error =
            Some(format!("Interface {new_interface} does not exist"));
        return;
    }
    // Change the interface
    app.interface_name = Some(new_interface);
}

/// Rescan for new interfaces that have been connected since the program started
pub(crate) fn rescan_interfaces() {
    let mut write_handle = INTERFACES.write();
    *write_handle = interfaces();
}
