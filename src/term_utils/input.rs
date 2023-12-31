use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::appstate::InputMode;

pub(crate) enum KeyHandlerEvents {
    None,
    Break,
    ToEditing,
    ToNormal,
    KeyPress(KeyEvent),
    SendMessage,
}

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
