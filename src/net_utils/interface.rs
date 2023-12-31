//! All hardware interface related functionality lives here

use std::{net::IpAddr, thread};

use anyhow::{Result, anyhow};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use pnet::{datalink::{
    self,
    dummy::interfaces,
    Channel::Ethernet,
    NetworkInterface,
}, packet::ethernet::EthernetPacket};

use crate::appstate::App;

/// A collection of interfaces connected to the system
static INTERFACES: Lazy<RwLock<Vec<NetworkInterface>>> =
    Lazy::new(|| RwLock::new(interfaces()));

static DETECTED_HOSTS: RwLock<Vec<IpAddr>> = RwLock::new(Vec::new());

/// Change the interface the program is using
pub(crate) fn change_interface(app: &mut App, new_interface: String) {
    let read_handle = INTERFACES.read();
    // Check if the selected interface is real
    let interface_names =
        read_handle.iter().map(|x| &x.name).collect::<Vec<&String>>();
    if !interface_names.contains(&&new_interface) {
        app.last_error =
            Some(format!("Interface {new_interface} does not exist"));
        return;
    }
    // Change the interface
    app.interface_name = Some(new_interface);
}

/// Rescan for new interfaces that have been connected since the program started
pub(crate) fn rescan_interfaces() {
    let mut write_handle = INTERFACES.write();
    *write_handle = interfaces();
}

/// Spawn a thread listening on a given interface
fn spawn_listener(
    interface: NetworkInterface,
) -> Result<thread::JoinHandle<()>> {
    let Ok(Ethernet(mut tx, mut rx)) =
            datalink::channel(&interface, Default::default())
        else {
            return Err(anyhow!("Unsupported interface type"));
        };
    let handle = thread::spawn(move || {
        let mut sources: Vec<
        loop {
            match rx.next() {
                Ok(packet) => {
                    let packet = EthernetPacket::new(packet).expect("God help us if the packet isn't a packet");
                    let source = packet.get_source();
                },
                Err(e) => panic!("Listener crashed"),
            }
        }
    });
    Ok(handle)
}

/// Kill the listener thread
fn kill_listener() -> Result<()> {
    todo!();
}

/// Retrieve a list of detected hosts from the listener thread
pub(crate) fn listen() -> Vec<IpAddr> {
    todo!();
}
