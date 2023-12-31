//! Listener thread related code lives here/// Holds the thread handle in global
//! scope to keep it from dropping

use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

use anyhow::{anyhow, Result};
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
) -> Result<Receiver<MacAddr>> {
    let Some(interface) = interface_from_name(&app.interface_name) else {
        panic!("Tried to start listener on nonexistent interface")
    };
    let Ok(Ethernet(_, mut interface_rx)) =
        datalink::channel(&interface, Config::default())
    else {
        return Err(anyhow!("Unsupported interface type"));
    };
    // Spawn the thread
    let (thread_tx, thread_rx) = std::sync::mpsc::channel();
    let handle = thread::spawn(move || loop {
        let mut discovered_hosts = Vec::new();
        let packet = interface_rx.next().expect("Listener crashed");
        std::fs::File::create("packet").unwrap();
        let packet = EthernetPacket::new(packet)
            .expect("God help us if the packet isn't a packet");
        let source = packet.get_source();
        thread_tx.send(source).expect("Thread transmission failed");
        if !discovered_hosts.contains(&source) {
            discovered_hosts.push(source);
            thread_tx.send(source).expect("Thread transmission failed");
        }
    });
    let mut write_handle = LISTENER_THREAD.write();
    *write_handle = Some(handle);
    Ok(thread_rx)
}

/// Kill the listener thread
pub(crate) fn kill_listener(app: &App) {
    let mut write_handle = LISTENER_THREAD.write();
    *write_handle = None;
}

/// Retrieve a list of detected hosts from the listener thread
pub(crate) fn listen(app: &App) {
    println!("Listening");
    if let Some(receiver) = &app.listen_thread_rx {
        for address in receiver.try_iter() {
            println!("{address:?}");
            app.new_mac(address);
        }
    }
}