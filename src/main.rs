#![doc = include_str!("../README.md")]

use actix::prelude::*;

mod ui;

fn main() {
    let ui_addr = ui::UiActor {}.start();
    ui_addr.do_send(ui::StartMessage);
    println!("Hewwo");
}