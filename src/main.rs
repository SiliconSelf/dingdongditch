#![doc = include_str!("../README.md")]

use std::io;

use crossterm::{
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod commands;
mod net;
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
    // Set up terminal for ratatui
    let mut stdout = io::stdout();
    enable_raw_mode().expect("Enabling raw mode should always succeed");
    execute!(stdout, EnterAlternateScreen)
        .expect("Entering an alternate screen should always succeed");
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))
        .expect("Creating a terminal should always succeed");

    logic_loop(&mut terminal);

    disable_raw_mode().expect("Disabling raw mode should always succeed.");
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .expect("Leaving the alternate screen should always succeeed");
    terminal.show_cursor().expect("Unable to show cursor");
}
