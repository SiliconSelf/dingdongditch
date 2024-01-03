//! Contains code for the Host struct
//!
//! See `net` module documentation for a broader overview

use std::{fmt::Display, net::IpAddr};

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

impl Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = self.mac_address.to_string();
        if let Some(ip) = self.ip_address {
            text = ip.to_string();
        };
        if let Some(name) = &self.domain_name {
            text = name.to_owned();
        };
        write!(f, "{text}")
    }
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
