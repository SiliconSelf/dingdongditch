//! Terminal input handler

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::appstate::InputMode;

/// The possible events output by the key handler
pub(crate) enum KeyHandlerEvents {
    /// Nothing to do
    None,
    /// Break the event loop
    Break,
    /// Switch to editing mode
    ToEditing,
    /// Switch to normal mode
    ToNormal,
    /// Handle a keypress in editing mode
    KeyPress(KeyEvent),
    /// Send a message
    SendMessage,
}

/// Handle keypresses from the user
pub(crate) fn handle_keys(mode: &InputMode) -> KeyHandlerEvents {
    if let Ok(Event::Key(key)) = event::read() {
        match mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('e') => KeyHandlerEvents::ToEditing,
                KeyCode::Char('q') => KeyHandlerEvents::Break,
                _ => KeyHandlerEvents::None,
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => KeyHandlerEvents::SendMessage,
                KeyCode::Esc => KeyHandlerEvents::ToNormal,
                _ => KeyHandlerEvents::KeyPress(key),
            },
        }
    } else {
        KeyHandlerEvents::None
    }
}
