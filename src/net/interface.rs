//! Functionality related to raw interface bindings and Layer 2 networking

use pnet::datalink::interfaces;

/// Get the name of the first plausible interface
///
/// A plausible interface means an interface has a MAC address, at least one
/// IPv4 address, is up, is not a loopback interface
pub(crate) fn find_plausible_interface() -> Option<String> {
    let interfaces = interfaces();
    let plausible_interface = interfaces.iter().find(|interface| {
        if interface.mac.is_none()
            || interface.ips.is_empty()
            || !interface.is_up()
            || interface.is_loopback()
        {
            return false;
        }
        let potential_ipv4 = interface.ips.iter().find(|ip| ip.is_ipv4());
        potential_ipv4.is_some()
    });
    plausible_interface.map(|i| i.name.clone())
}
