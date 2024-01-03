//! The textbox for entering commands
//!
//! See `elements` module documentation for more detail

use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, InputMode};

/// Create the input box
pub(crate) fn input_element(scroll: usize, app: &App) -> Paragraph<'_> {
    // This should probably be handled better, but I don't forsee commands
    // longer than u16::MAX
    #[allow(clippy::as_conversions)]
    #[allow(clippy::cast_possible_truncation)]
    let cast_scroll = scroll as u16;

    let input = Paragraph::new(app.get_input().value())
        .style(match app.get_input_mode() {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .scroll((0, cast_scroll))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    input
}
