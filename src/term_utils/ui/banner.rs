use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::Paragraph,
};

use crate::appstate::App;

pub(crate) fn banner_element(app: &App) -> Paragraph<'_> {
    let (banner_text, banner_style) = (
        vec![
            Span::raw("Interface: "),
            Span::styled(
                if let Some(i) = app.interface_name.clone() {
                    i
                } else {
                    "None".to_owned()
                },
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
        ],
        Style::default().add_modifier(Modifier::RAPID_BLINK),
    );
    let mut text = Text::from(Line::from(banner_text));
    text.patch_style(banner_style);
    Paragraph::new(text)
}
