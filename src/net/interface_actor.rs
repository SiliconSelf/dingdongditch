//! Contains code related to an actor for a specific interface

use std::collections::HashSet;

use actix::prelude::*;
use pnet::{
    datalink::{self, Channel::Ethernet, Config, NetworkInterface},
    packet::ethernet::EthernetPacket,
    util::MacAddr,
};

use super::{DetectedHost, InterfaceManagerActor, NetworkInterfaceRequest};
use crate::net::NewHostMessage;

/// An actor for a specific interface
pub(crate) struct InterfaceActor {
    /// The address of the manager actor
    manager_addr: Addr<InterfaceManagerActor>,
    /// What interface the actor should use
    interface: NetworkInterface,
}

impl InterfaceActor {
    /// Create a new actor
    pub(crate) fn new(manager_addr: Addr<InterfaceManagerActor>) -> Self {
        log::trace!("Creating new interface actor");
        let interface = futures::executor::block_on(async {
            manager_addr
                .send(NetworkInterfaceRequest)
                .await
                .expect("Failed to request network interface")
                .expect("No free network interfaces")
        });
        Self {
            manager_addr,
            interface,
        }
    }
}

impl Actor for InterfaceActor {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        log::trace!("Interface actor started for {}", self.interface.name);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::trace!("Interface actor stopped for {}", self.interface.name);
    }
}

/// Tell a interface actor to start listening for frames
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub(crate) struct Listen;

impl Handler<Listen> for InterfaceActor {
    type Result = ();

    fn handle(
        &mut self,
        msg: Listen,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        log::trace!(
            "InterfaceActor for {} received {msg:?}",
            self.interface.name
        );
        let (_, mut interface_rx) =
            match datalink::channel(&self.interface, Config::default()) {
                Ok(Ethernet(tx, rx)) => (tx, rx),
                Ok(_) => panic!("Unsupported channel type"),
                Err(e) => panic!("Unhandled error: {e:?}"),
            };
        let mut detected_macs = HashSet::new();
        loop {
            let packet = interface_rx.next().expect("Listener crashed");
            // I need to do less heap allocations
            if let Some(packet) = EthernetPacket::new(packet) {
                let source = packet.get_source();
                if detected_macs.insert(source) {
                    self.manager_addr.do_send(NewHostMessage {
                        interface_name: self.interface.name.clone(),
                        address: source,
                    });
                }
            }
        }
    }
}
