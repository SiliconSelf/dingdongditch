use crate::commands::{self, Command};

#[test]
fn q_command() {
    assert_eq!(commands::parse_command("q".to_owned()), Some(Command::Quit));
}

#[test]
fn quit_command() {
    assert_eq!(commands::parse_command("quit".to_owned()), Some(Command::Quit))
}

#[test]
fn quit_command_rejects() {
    assert_eq!(commands::parse_command("qs".to_owned()), None);
    assert_eq!(commands::parse_command("quits".to_owned()), None);
    assert_eq!(commands::parse_command("quit 1".to_owned()), None);
}