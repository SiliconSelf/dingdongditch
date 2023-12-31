//! All hardware interface related functionality lives here

use pnet::datalink::{interfaces, NetworkInterface};

use crate::appstate::App;

/// Change the interface the program is using
pub(crate) fn change_interface(app: &mut App, new_interface: String) {
    let interfaces = interfaces();
    if !interfaces.iter().any(|x| x.name == new_interface) {
        app.last_error =
            Some(format!("Interface {new_interface} does not exist"));
        return;
    }
    // Change the interface
    app.interface_name = new_interface;
}

/// Get an interface from its name as a String
pub(crate) fn interface_from_name(
    interface_name: &str,
) -> Option<NetworkInterface> {
    let interfaces = interfaces();
    interfaces.iter().find(|f| f.name == interface_name).cloned()
}
