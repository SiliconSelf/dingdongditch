//! Banner at the top of the UI.
//!
//! See `elements` module documentation for more details.

use ratatui::{
    style::{Style, Modifier},
    text::{Line, Span, Text},
    widgets::Paragraph,
};

use crate::app::App;

/// Creates the banner element
pub(crate) fn banner_element(app: &App) -> Paragraph<'_> {
    let (banner_text, banner_style) = (
        vec![
            Span::raw("Interface: "),
            Span::styled(app.get_interface_name(), Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::raw("Listening: "),
            Span::styled("false", Style::default().add_modifier(Modifier::BOLD).fg(ratatui::style::Color::Red))
        ],
        Style::default(),
    );
    let mut text = Text::from(Line::from(banner_text));
    text.patch_style(banner_style);
    Paragraph::new(text)
}
