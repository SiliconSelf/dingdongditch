#![doc = include_str!("../README.md")]

use std::{collections::VecDeque, io};

use app::APP_STATE;
use commands::{Command, Errors};
use crossterm::{
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use net::Host;
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
        // Get new data from listener
        if let Some(new_addresses) = net::listen() {
            let mut write_handle = APP_STATE.write();
            for new in new_addresses {
                write_handle.add_host(Host::new(new));
            }
        }
        // Draw UI
        let Ok(_) = terminal.draw(ui::render_ui) else {
            break;
        };
        // Handle user input
        key_handler::handle_keys();
        // Process any user commands
        let mut write_handle = APP_STATE.write();
        let shared_commands = write_handle.get_commands_mut();
        if !shared_commands.is_empty() {
            let commands: VecDeque<String> = std::mem::take(shared_commands);
            for command in commands {
                let command_result = Command::try_from(command.clone());
                if command_result.is_ok() {
                    write_handle.last_error(None);
                }
                match command_result {
                    Ok(Command::Quit) => {
                        return;
                    }
                    Ok(Command::Listen) => {
                        write_handle.toggle_listening();
                    }
                    Ok(Command::ChangeInterface(i)) => {
                        match write_handle.interface_name(&i) {
                            Ok(()) => {}
                            Err(app::Errors::NoSuchInterface) => write_handle
                                .last_error(Some(format!(
                                    "No such interface: {i}"
                                ))),
                        }
                    }
                    Ok(Command::Select(i)) => {
                        let all_hosts = write_handle.get_hosts();
                        if all_hosts.len() > i {
                            write_handle.selected_host(i);
                        } else {
                            write_handle.last_error(Some(format!(
                                "Specified host out of range: {i}"
                            )));
                        }
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

    // Restore terminal to its original state
    disable_raw_mode().expect("Disabling raw mode should always succeed.");
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .expect("Leaving the alternate screen should always succeeed");
    terminal.show_cursor().expect("Unable to show cursor");
}
