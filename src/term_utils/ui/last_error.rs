use ratatui::{
    style::{Modifier, Style, Color},
    text::{Line, Span, Text},
    widgets::Paragraph,
};

use crate::appstate::App;

pub(crate) fn error_element(app: &App) -> Paragraph<'_> {
    let (banner_text, banner_style) = (
        vec![
            Span::styled(
                if let Some(i) = app.last_error.clone() {
                    i
                } else {
                    String::new()
                },
                Style::default().fg(Color::Red),
            ),
            Span::raw(" "),
        ],
        Style::default().add_modifier(Modifier::RAPID_BLINK),
    );
    let mut text = Text::from(Line::from(banner_text));
    text.patch_style(banner_style);
    Paragraph::new(text)
}
