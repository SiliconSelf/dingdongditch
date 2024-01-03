//! Displays the last error
//! 
//! See `elements` module documentation for more details

use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::Paragraph,
};

use crate::app::App;

/// Function to create the element
pub(crate) fn last_error_element(app: &App) -> Paragraph<'_> {
    let (banner_text, banner_style) = (
        vec![
            Span::styled(
                if let Some(i) = app.get_last_error() {
                    i
                } else {
                    ""
                },
                Style::default().fg(Color::Red),
            ),
            Span::raw(" "),
        ],
        Style::default(),
    );
    let mut text = Text::from(Line::from(banner_text));
    text.patch_style(banner_style);
    Paragraph::new(text)
}