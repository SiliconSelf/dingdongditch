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

/// Try to find the default network interface of the system
pub(crate) fn find_default_interface() -> NetworkInterface {
    let interfaces = interfaces();
    let default_interface = interfaces.iter().find(|interface| {
        if interface.mac.is_none() || interface.ips.is_empty() || !interface.is_up() || interface.is_loopback() {
            return false;
        }
        let potential_ipv4 = interface.ips.iter().find(|ip| ip.is_ipv4());
        potential_ipv4.is_some()
    });
    default_interface.cloned().expect("Surely you have a networking interface")
}