//! The manager actor is a collector for messages from the various
//! `InterfaceActor`s

use std::collections::HashMap;

use actix::prelude::*;
use parking_lot::Mutex;
use pnet::{datalink::NetworkInterface, util::MacAddr};

use super::{get_interfaces, DetectedHost};

/// A manager actor. See module documentation for more details.
pub(crate) struct InterfaceManagerActor {
    /// A collection of network interfaces that have not been returned to an
    /// individual interface actor
    interfaces: Mutex<Vec<NetworkInterface>>,
    /// A collection of the hosts detected by various interfaces
    detected_hosts: HashMap<String, Vec<DetectedHost>>,
}

impl Actor for InterfaceManagerActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::trace!("Started interface manager actor");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::trace!("Stopped interface manager actor");
    }
}

impl InterfaceManagerActor {
    /// Create a new manager actor
    pub(crate) fn new() -> Self {
        let interfaces = get_interfaces();
        log::debug!("Detected {} sensible interfaces", interfaces.len());
        let mut detected_hosts = HashMap::new();
        for interface in &interfaces {
            detected_hosts.insert(interface.name.clone(), Vec::new());
        }
        let interfaces = Mutex::new(interfaces);
        Self {
            interfaces,
            detected_hosts,
        }
    }
}

/// A request from an `InterfaceActor` for a network interface to assign itself
/// to. Returns `None` if all interfaces have been assigned.
#[derive(Message, Debug)]
#[rtype(result = "Option<NetworkInterface>")]
pub(crate) struct NetworkInterfaceRequest;

impl Handler<NetworkInterfaceRequest> for InterfaceManagerActor {
    type Result = Option<NetworkInterface>;

    fn handle(
        &mut self,
        msg: NetworkInterfaceRequest,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        log::trace!("InterfaceManagerActor received {msg:?}");
        let mut handle = self.interfaces.lock();
        handle.pop()
    }
}

/// A request for how many unassigned network interfaces there are
#[derive(Message, Debug)]
#[rtype(result = "usize")]
pub(crate) struct NetworkInterfaceCountRequest;

impl Handler<NetworkInterfaceCountRequest> for InterfaceManagerActor {
    type Result = usize;

    fn handle(
        &mut self,
        msg: NetworkInterfaceCountRequest,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        log::trace!("InterfaceManagerActor received {msg:?}");
        let handle = self.interfaces.lock();
        handle.len()
    }
}

/// A query for if new data has arrived
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub(crate) struct NewDataQuery;

impl Handler<NewDataQuery> for InterfaceManagerActor {
    type Result = ();

    fn handle(
        &mut self,
        _msg: NewDataQuery,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        // log::trace!("InterfaceManagerActor received {msg:?}");
    }
}

/// An alert that a new host has been detected
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub(crate) struct NewHostMessage {
    /// The interface the host was detected on
    pub(crate) interface_name: String,
    /// The mac address of the detected host
    pub(crate) address: MacAddr,
}

impl Handler<NewHostMessage> for InterfaceManagerActor {
    type Result = ();

    fn handle(
        &mut self,
        msg: NewHostMessage,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        log::trace!("InterfaceManagerActor received {msg:?}");
        let detections = self
            .detected_hosts
            .get_mut(&msg.interface_name)
            .expect("Interface not defined in hashmap");
        let new_host = DetectedHost::new(msg.address);
        detections.push(new_host);
    }
}
