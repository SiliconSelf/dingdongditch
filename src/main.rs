#![doc = include_str!("../README.md")]

use actix::prelude::*;
// use ui::StartMessage;

mod net;
mod ui;

use net::InterfaceActor;

#[actix::main]
async fn main() {
    simple_logger::init().expect("Failed to initialize logging");
    let ui_arbiter = Arbiter::new();
    ui_arbiter.spawn(async move {
        let ui_actor = ui::UiActor {}.start();
        // ui_actor
        //     .send(StartMessage)
        //     .await
        //     .expect("UI Actor panicked")
        //     .expect("IO Error");
        // System::current().stop();
    });
    let interfaces = net::get_interfaces();
    let interface_count = interfaces.len();
    if interface_count == 0 {
        log::error!("There are no attached interfaces");
        System::current().stop();
    }
    log::debug!("Detected {interface_count} sensible interfaces");
    let adapter_arbiter =
        SyncArbiter::start(interface_count, InterfaceActor::new);
    std::thread::sleep(std::time::Duration::from_secs(2));
}
