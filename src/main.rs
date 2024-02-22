#![doc = include_str!("../README.md")]

use actix::prelude::*;
use ui::StartMessage;

mod ui;

#[actix::main]
async fn main() {
    let ui_arbiter = Arbiter::new();
    ui_arbiter.spawn(async move {
        let ui_actor = ui::UiActor {}.start();
        ui_actor.send(StartMessage).await.expect("UI Actor panicked").expect("IO Error");
        System::current().stop();
    });

}