//! Contains code for the Host struct
//!
//! See `net` module documentation for a broader overview

use std::{fmt::Display, net::IpAddr};

use pnet::util::MacAddr;

/// A structure representing a specific host detected on the network
#[derive(PartialEq, Eq)]
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
        if let Some(domain) = self.domain_name.clone() {
            return write!(f, "{domain}");
        } else if let Some(ip) = self.ip_address {
            return write!(f, "{ip}");
        }
        write!(f, "{}", self.mac_address)
    }
}

impl Host {
    /// Creates a new host
    pub(crate) fn new(mac_address: MacAddr) -> Self {
        Self {
            mac_address,
            ip_address: None,
            domain_name: None,
            ports: None,
        }
    }

    /// Get a reference to the mac address
    pub(crate) fn get_mac_address(&self) -> &MacAddr {
        &self.mac_address
    }

    /// Get a reference to the IP address
    pub(crate) fn get_ip_address(&self) -> &Option<IpAddr> {
        &self.ip_address
    }

    /// Get a reference to the domain name
    pub(crate) fn get_domain_name(&self) -> &Option<String> {
        &self.domain_name
    }

    /// Get a reference to the open ports
    pub(crate) fn get_ports(&self) -> &Option<Vec<u16>> {
        &self.ports
    }
}
