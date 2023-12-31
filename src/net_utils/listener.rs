//! Listener thread related code lives here/// Holds the thread handle in global
//! scope to keep it from dropping

use std::thread::{self, JoinHandle};

use crossbeam_channel::Receiver;
use parking_lot::RwLock;
use pnet::{
    datalink::{self, Channel::Ethernet, Config},
    packet::ethernet::EthernetPacket,
    util::MacAddr,
};

use super::interface::interface_from_name;
use crate::appstate::App;

/// Holds the join handle for the listener thread in global scope so it isn't
/// dropped
static LISTENER_THREAD: RwLock<Option<JoinHandle<()>>> = RwLock::new(None);

/// Spawn a thread listening on a given interface
pub(crate) fn spawn_listener(
    app: &App
) -> Receiver<MacAddr> {
    let Some(interface) = interface_from_name(&app.interface_name) else {
        panic!("Tried to start listener on nonexistent interface")
    };
    let (_, mut interface_rx) = match datalink::channel(&interface, Config::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => { panic!("Unsupported channel type") },
        Err(e) => { panic!("Can't create datalink channel {e}") }
    };
    // Spawn the thread
    let (thread_tx, thread_rx) = crossbeam_channel::unbounded();
    let handle = thread::spawn(move || loop {
        let mut discovered_hosts = Vec::new();
        let packet = interface_rx.next().expect("Listener crashed");
        let packet = EthernetPacket::new(packet)
            .expect("God help us if the packet isn't a packet");
        let source = packet.get_source();
        if !discovered_hosts.contains(&source) {
            discovered_hosts.push(source);
            thread_tx.send(source).expect("Thread transmission failed");
        }
    });
    let mut write_handle = LISTENER_THREAD.write();
    *write_handle = Some(handle);
    thread_rx
}

/// Kill the listener thread
pub(crate) fn kill_listener(app: &App) {
    let mut write_handle = LISTENER_THREAD.write();
    *write_handle = None;
}

/// Retrieve a list of detected hosts from the listener thread
pub(crate) fn listen(app: &App) {
    if let Some(receiver) = &app.listen_thread_rx {
        for address in receiver.try_iter() {
            app.new_mac(address);
        }
    }
}