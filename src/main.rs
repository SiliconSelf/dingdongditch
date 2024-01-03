#![doc = include_str!("../README.md")]

use std::io;

use app::APP_STATE;
use commands::{Command, Errors};
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
mod key_handler;
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
        // Handle user input
        key_handler::handle_keys();
        // TODO: Process any user commands
        let mut write_handle = APP_STATE.write();
        let shared_commands = write_handle.get_commands_mut();
        if !shared_commands.is_empty() {
            println!("Commands not empty");
            let commands: Vec<String> = std::mem::take(shared_commands);
            for command in commands {
                match Command::try_from(command.clone()) {
                    Ok(Command::Quit) => {
                        println!("Called quit");
                        return;
                    }
                    Err(Errors::UnknownCommand) => {
                        write_handle.last_error(Some(format!(
                            "Unknown Command: {command}"
                        )));
                    }
                }
            }
        }
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

    // Run the main program logic loop
    logic_loop(&mut terminal);
    println!("Loop returned");

    // Restore terminal to its original state
    disable_raw_mode().expect("Disabling raw mode should always succeed.");
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .expect("Leaving the alternate screen should always succeeed");
    terminal.show_cursor().expect("Unable to show cursor");
}
