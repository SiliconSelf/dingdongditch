//! Functionality related to networking

use actix::prelude::*;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use pnet::datalink::{interfaces, NetworkInterface};

/// A list of all sensible interfaces for initialization of actors.
static INTERFACES: Lazy<Mutex<Vec<NetworkInterface>>> =
    Lazy::new(|| Mutex::new(get_interfaces()));

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

/// An actor for a specific interface
pub(crate) struct InterfaceActor {
    /// What interface the actor should use
    interface: NetworkInterface,
}

impl InterfaceActor {
    /// Create a new actor
    pub(crate) fn new() -> Self {
        let mut handle = INTERFACES.lock();
        let interface = handle.pop().expect("No interface");
        Self {
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

/// A message for an actor to take an interface
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub(crate) struct TakeInterfaceMesssage;

impl Handler<TakeInterfaceMesssage> for InterfaceActor {
    type Result = ();

    fn handle(
        &mut self,
        msg: TakeInterfaceMesssage,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        log::trace!(
            "Interface actor for {} received {msg:?}",
            self.interface.name
        );
    }
}
