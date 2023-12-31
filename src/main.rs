#![doc = include_str!("../README.md")]

use std::io;

use appstate::{App, InputMode};
use crossterm::event::Event;
use ratatui::{backend::Backend, Terminal};
use term_utils::{handle_keys, restore_terminal, setup_terminal};
use tui_input::backend::crossterm::EventHandler;

mod appstate;
mod commands;
mod net;
mod term_utils;

#[cfg(test)]
mod tests;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut terminal = setup_terminal().context("Failed to set up terminal")?;

    // Create the global app state and run the main logic loop until it returns
    let app = App::default();
    run_app(&mut terminal, app)?;

    restore_terminal(&mut terminal)?;
    Ok(())
}

/// Main logic loop
fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| term_utils::ui(f, &app))?;
        // Handle user input
        match handle_keys(&app.input_mode) {
            term_utils::KeyHandlerEvents::None => {}
            term_utils::KeyHandlerEvents::Break => return Ok(()),
            term_utils::KeyHandlerEvents::ToEditing => {
                app.input_mode = InputMode::Editing
            }
            term_utils::KeyHandlerEvents::ToNormal => {
                app.input_mode = InputMode::Normal
            }
            term_utils::KeyHandlerEvents::KeyPress(key) => {
                app.input.handle_event(&Event::Key(key));
            }
            term_utils::KeyHandlerEvents::SendMessage => {
                app.messages.push(app.input.value().into());
                app.input.reset();
            }
        }
        // Process any new user commands
        if !app.messages.is_empty() {
            for input in app.messages.drain(..) {
                if let Some(command) = commands::parse_command(input) {
                    use commands::Command;
                    match command {
                        Command::Quit => return Ok(()),
                        Command::ChangeInterface(interface) => { app.interface_name = Some(interface) }
                    }
                }
            }
        }
    }
}
