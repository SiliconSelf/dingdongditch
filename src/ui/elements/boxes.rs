//! Contains the box elements in the middle of the screen for displaying hosts
//! and details
//!
//! See `elements` module documentation for more details

use ratatui::{
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::App;

///
pub(crate) fn hosts_box_element(app: &App) -> List<'_> {
    let hosts: Vec<ListItem> = app
        .get_hosts()
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = vec![Line::from(Span::raw(format!("{i}: {m}")))];
            ListItem::new(content)
        })
        .collect();
    let hosts = List::new(hosts)
        .block(Block::default().borders(Borders::ALL).title("Hosts"));
    hosts
}

///
pub(crate) fn details_box_element(_app: &App) -> List<'_> {
    let details: Vec<ListItem> = Vec::new();
    List::new(details)
        .block(Block::default().borders(Borders::ALL).title("Details"))
}
