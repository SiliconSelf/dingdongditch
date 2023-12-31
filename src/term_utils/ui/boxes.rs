//! Structures and functions related to the boxes

use ratatui::{
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

use crate::appstate::App;

/// Create the box that contains the list of detected hosts
pub(crate) fn hosts_box_element(app: &App) -> List<'_> {
    let hosts: Vec<ListItem> = app
        .hosts
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

/// Create the box that shows the open ports on a given host
pub(crate) fn ports_box_element(_app: &App) -> List<'_> {
    let ports: Vec<ListItem> = Vec::new();
    let ports = List::new(ports)
        .block(Block::default().borders(Borders::ALL).title("Ports"));
    ports
}
