//! Contains the box elements in the middle of the screen for displaying hosts
//! and details
//!
//! See `elements` module documentation for more details

use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::App;

///
pub(crate) fn hosts_box_element(app: &App) -> List<'_> {
    let selected_host = app.get_selected_host();
    let hosts: Vec<ListItem> = app
        .get_hosts()
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let style = if &i == selected_host {
                Style::default().fg(ratatui::style::Color::Green)
            } else {
                Style::default()
            };
            let content = Line::from(Span::styled(format!("{i}: {m}"), style));
            ListItem::new(content)
        })
        .collect();
    let hosts = List::new(hosts)
        .block(Block::default().borders(Borders::ALL).title("Hosts"));
    hosts
}

/// Display the details of the currently selected host
pub(crate) fn details_box_element(app: &App) -> List<'_> {
    let current_index = app.get_selected_host().to_owned();
    let details: Vec<ListItem> = {
        if let Some(current_host) = app.get_hosts().get(current_index) {
            let mut content = vec![
                ListItem::new(format!(
                    "MAC Address: {}",
                    current_host.get_mac_address()
                )),
                ListItem::new(format!(
                    "IP Address: {}",
                    match current_host.get_ip_address() {
                        Some(ip) => ip.to_string(),
                        None => "Unknown".to_owned(),
                    }
                )),
                ListItem::new(format!(
                    "Domain Name: {}",
                    match current_host.get_domain_name() {
                        Some(domain) => domain.to_owned(),
                        None => "Unknown".to_owned(),
                    }
                )),
                ListItem::new(String::new()),
                ListItem::new("Open Ports:".to_owned()),
            ];
            if let Some(ports) = current_host.get_ports() {
                for port in ports {
                    content.push(ListItem::new(format!("- {port}")));
                }
            }
            content
        } else {
            Vec::new()
        }
    };
    List::new(details)
        .block(Block::default().borders(Borders::ALL).title("Details"))
}
