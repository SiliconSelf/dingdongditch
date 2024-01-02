#![doc = include_str!("../README.md")]

use ratatui::{backend::Backend, Terminal};

mod app;
mod commands;
mod net;
mod term_utils;
mod ui;

/// The main logic loop of the app
fn logic_loop<B: Backend>(terminal: &mut Terminal<B>) {
    loop {
        // TODO: Get new data from listeners
        // Draw UI
        let Ok(_) = terminal.draw(ui::render_ui) else {
            break;
        };
        // TODO: Process any user commands
        todo!();
    }
}

fn main() {
    let mut terminal =
        term_utils::setup_terminal().expect("Failed to configure terminal");

    logic_loop(&mut terminal);

    term_utils::restore_terminal(&mut terminal).expect(
        "Failed to restore terminal to its original state. Your terminal is \
         probably broken and needs to be restarted.",
    );
}
