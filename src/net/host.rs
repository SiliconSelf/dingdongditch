//! Contains code for the Host struct
//!
//! See `net` module documentation for a broader overview

use std::net::IpAddr;

use pnet::util::MacAddr;

/// A structure representing a specific host detected on the network
#[derive(PartialEq, Eq, Hash)]
pub(crate) struct Host {
    /// The MAC address of the host
    mac_address: MacAddr,
    /// The IP address of the host, optionally discovered through ARP
    ip_address: Option<IpAddr>,
    /// The local domain name of the host, optionally discovered through DNS
    domain_name: Option<String>,
    /// The open ports of the host. None if the host hasn't been scanned
    ports: Option<Vec<u16>>,
}

impl Host {
    fn new(mac_address: MacAddr) -> Self {
        Self {
            mac_address,
            ip_address: None,
            domain_name: None,
            ports: None,
        }
    }
}
