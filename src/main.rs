#![doc = include_str!("../README.md")]

use actix::prelude::*;
use ui::StartMessage;

mod net;
mod ui;

use net::{
    InterfaceActor, InterfaceManagerActor, Listen,
    NetworkInterfaceCountRequest, NewDataQuery,
};

#[actix::main]
async fn main() {
    // simple_logger::init().expect("Failed to initialize logging");
    let ui_arbiter = Arbiter::new();
    ui_arbiter.spawn(async move {
        let ui_actor = ui::UiActor {}.start();
        ui_actor
            .send(StartMessage)
            .await
            .expect("UI Actor panicked")
            .expect("IO Error");
        System::current().stop();
    });
    let interface_actor_manager = InterfaceManagerActor::new().start();
    let interface_count = interface_actor_manager
        .send(NetworkInterfaceCountRequest)
        .await
        .expect("Failed to query number of network interfaces");
    let new_addr = interface_actor_manager.clone();
    let adapter_arbiter = SyncArbiter::start(interface_count, move || {
        InterfaceActor::new(new_addr.clone())
    });
    for _ in 0..interface_count {
        adapter_arbiter.do_send(Listen);
    }
    loop {
        interface_actor_manager
            .send(NewDataQuery)
            .await
            .expect("This really doesn't matter");
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}
