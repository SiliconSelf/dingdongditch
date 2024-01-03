//! Banner at the top of the UI.
//!
//! See `elements` module documentation for more details.

use ratatui::{
    style::Style,
    text::{Line, Span, Text},
    widgets::Paragraph,
};

use crate::app::App;

/// Creates the banner element
pub(crate) fn banner_element(_app: &App) -> Paragraph<'_> {
    let (banner_text, banner_style) = (
        vec![
            Span::raw("Interface: "),
            Span::raw(" "),
            Span::raw("Listening: "),
        ],
        Style::default(),
    );
    let mut text = Text::from(Line::from(banner_text));
    text.patch_style(banner_style);
    Paragraph::new(text)
}
