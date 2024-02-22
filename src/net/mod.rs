//! Functionality related to networking
use pnet::{datalink::{interfaces, NetworkInterface}, util::MacAddr};

mod interface_actor;
mod manager_actor;

pub(crate) use interface_actor::*;
pub(crate) use manager_actor::*;

/// Get sensible interfaces
///
/// A sensible interface is up, not a loopback, and has at least one IP address
pub(crate) fn get_interfaces() -> Vec<NetworkInterface> {
    let all_interfaces = interfaces();
    let sensible_interfaces: Vec<NetworkInterface> = all_interfaces
        .iter()
        .filter(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty())
        .map(std::borrow::ToOwned::to_owned)
        .collect();
    sensible_interfaces
}

pub(crate) struct DetectedHost {
    mac_address: MacAddr
}

impl DetectedHost {
    pub(crate) fn new(mac_address: MacAddr) -> Self {
        Self {
            mac_address
        }
    }
}