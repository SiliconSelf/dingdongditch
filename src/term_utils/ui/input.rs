//! Structures and functions related to the input textbox

use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::appstate::{App, InputMode};

/// Create the input box
pub(crate) fn input_element(scroll: usize, app: &App) -> Paragraph<'_> {
    // This should probably be handled better, but I don't forsee commands
    // longer than u16::MAX
    #[allow(clippy::as_conversions)]
    #[allow(clippy::cast_possible_truncation)]
    let cast_scroll = scroll as u16;

    let input = Paragraph::new(app.input.value())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .scroll((0, cast_scroll))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    input
}
