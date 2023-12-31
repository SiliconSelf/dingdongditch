use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::appstate::{App, InputMode};

mod banner;
mod boxes;
mod input;
mod last_error;

pub(crate) fn ui(f: &mut Frame, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [Constraint::Length(1), Constraint::Min(1), Constraint::Length(1), Constraint::Length(3)]
                .as_ref(),
        )
        .split(f.size());

    let box_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .vertical_margin(1)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(main_chunks[1]);

    // Render banner
    f.render_widget(banner::banner_element(app), main_chunks[0]);

    // Render boxes
    f.render_widget(boxes::hosts_box_element(app), box_chunks[0]);
    f.render_widget(boxes::ports_box_element(app), box_chunks[1]);

    // Render last error
    f.render_widget(last_error::error_element(app), main_chunks[2]);

    // Render input box
    let width = main_chunks[3].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    let scroll = app.input.visual_scroll(width as usize);
    f.render_widget(input::input_element(scroll, app), main_chunks[3]);
    match app.input_mode {
        // Hide the cursor. `Frame` does this by default, so we don't need to do
        // anything here
        InputMode::Normal => {}
        // Make the cursor visible and ask tui-rs to put it at the specified
        // coordinates after rendering
        InputMode::Editing => {
            f.set_cursor(
                // Put cursor past the end of the input text
                main_chunks[3].x
                    + ((app.input.visual_cursor()).max(scroll) - scroll) as u16
                    + 1,
                // Move one line down, from the border to the input line
                main_chunks[3].y + 1,
            )
        }
    }
}
