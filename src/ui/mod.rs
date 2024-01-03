//! Drawing the terminal UI of the program

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::APP_STATE;

mod elements;

/// Render the main App UI
pub(crate) fn render_ui(frame: &mut Frame) {
    // Calculate the main overall layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(frame.size());

    // Calculate the boxes for results
    let box_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(main_chunks[1]);

    // Access global app state for rendering
    let read_handle = &*APP_STATE.read();
    // Render banner
    frame.render_widget(elements::banner_element(read_handle), main_chunks[0]);
    // Render boxes
    frame.render_widget(elements::hosts_box_element(read_handle), box_chunks[0]);
    frame.render_widget(elements::details_box_element(read_handle), box_chunks[1]);
    // Render last error
    frame.render_widget(elements::last_error_element(read_handle), main_chunks[2]);
    // Render input box
    let width = main_chunks[3].width.max(3) - 3; // keep 2 for borders and 1 for cursor
    #[allow(clippy::as_conversions)]
    let scroll = read_handle.get_input().visual_scroll(width as usize);
    frame.render_widget(elements::input_element(scroll, read_handle), main_chunks[3]);
}
