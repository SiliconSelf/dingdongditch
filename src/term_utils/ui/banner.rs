//! Structures and functions related to the banner element at the top of the
//! screen

use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::Paragraph,
};

use crate::appstate::App;

/// Create the banner element for the top of the screen
pub(crate) fn banner_element(app: &App) -> Paragraph<'_> {
    let (banner_text, banner_style) = (
        vec![
            Span::raw("Interface: "),
            Span::styled(
                &app.interface_name,
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::raw("Listening: "),
            Span::styled(
                format!("{}", app.listening),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ],
        Style::default()
    );
    let mut text = Text::from(Line::from(banner_text));
    text.patch_style(banner_style);
    Paragraph::new(text)
}
