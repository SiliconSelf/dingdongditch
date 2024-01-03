//! Handles keyboard input from the users

use crossterm::event::{self, Event, KeyCode};
use tui_input::backend::crossterm::EventHandler;

use crate::app::{InputMode, APP_STATE};

/// Handle keypresses from the user
pub(crate) fn handle_keys() {
    if let Ok(Event::Key(key)) = event::read() {
        let mut write_handle = APP_STATE.write();
        match write_handle.get_input_mode() {
            InputMode::Normal => {
                if matches!(key.code, KeyCode::Char('e')) {
                    write_handle.input_mode(InputMode::Editing);
                }
            }
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    // Unfortunately we need to clone here to free up the
                    // mutable reference to write_handle. I would like to find a
                    // better way to do this at some point.
                    let input = write_handle.get_input_mut();
                    let value = input.value().to_owned();
                    input.reset();
                    write_handle.push_command(&value);
                }
                KeyCode::Esc => {
                    write_handle.input_mode(InputMode::Normal);
                }
                _ => {
                    let handle = write_handle.get_input_mut();
                    handle.handle_event(&Event::Key(key));
                }
            },
        };
    }
}
