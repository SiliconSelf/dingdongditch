//! Listener thread for passively detecting Level 2 packets

use std::{
    collections::HashSet,
    thread::{spawn, JoinHandle},
};

use crossbeam_channel::Receiver;
use parking_lot::RwLock;
use pnet::{
    datalink::{self, Channel::Ethernet, Config},
    packet::ethernet::EthernetPacket,
    util::MacAddr,
};

use super::interface::interface_from_name;
use crate::app::APP_STATE;

/// Holds the join handle in global scope to keep it from dropping
static LISTENER_THREAD: RwLock<Option<JoinHandle<()>>> = RwLock::new(None);
/// Holds the Receiver in global scope to keep it from dropping
static LISTENER_THREAD_RX: RwLock<Option<Receiver<MacAddr>>> =
    RwLock::new(None);

/// Spawn listener thread
pub(crate) fn spawn_listener() -> Receiver<MacAddr> {
    let read_handle = APP_STATE.read();
    let interface = read_handle.get_interface_name();
    // FIXME: Don't use unwrap here
    let interface = interface_from_name(interface).unwrap();
    let (_, mut interface_rx) =
        match datalink::channel(&interface, Config::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => {
                panic!("Unsupported channel type");
            }
            Err(e) => {
                panic!("Unknown error {e}");
            }
        };
    let (thread_tx, thread_rx) = crossbeam_channel::unbounded();
    let handle = spawn(move || {
        let mut discovered_hosts = HashSet::new();
        loop {
            let packet = interface_rx.next().expect("Listener crashed");
            if let Some(packet) = EthernetPacket::new(packet) {
                let source = packet.get_source();
                if discovered_hosts.insert(source) {
                    thread_tx.send(source).expect("Main thread hung up");
                }
            }
        }
    });
    let mut write_handle = LISTENER_THREAD.write();
    *write_handle = Some(handle);
    thread_rx
}

/// Kill the currently running listener thread
pub(crate) fn kill_listener() {
    let mut write_handle = LISTENER_THREAD.write();
    *write_handle = None;
    let mut write_handle = LISTENER_THREAD_RX.write();
    *write_handle = None;
}

/// Make sure the listener is stopped or running according to the app state
fn start_stop_listener() {
    let (listener_enabled, listener_running) = {
        let app_read_handle = APP_STATE.read();
        let thread_read_handle = LISTENER_THREAD.read();
        (app_read_handle.get_listening(), thread_read_handle.is_some())
    };
    if listener_enabled && !listener_running {
        let mut write_handle = LISTENER_THREAD_RX.write();
        let thread_receiver = spawn_listener();
        *write_handle = Some(thread_receiver);
    }
    if !listener_enabled && listener_running {
        kill_listener();
    }
}

/// Retrieve new hosts from the current listener thread
pub(crate) fn listen() -> Option<Vec<MacAddr>> {
    start_stop_listener();
    let read_handle = LISTENER_THREAD_RX.read();
    if let Some(receiver) = &*read_handle {
        let receiver = receiver.to_owned();
        if !receiver.is_empty() {
            let new_hosts: Vec<MacAddr> = receiver.try_iter().collect();
            return Some(new_hosts);
        }
    }
    None
}
