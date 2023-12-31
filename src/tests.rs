use crate::commands::{self, Command};

#[test]
fn q_command() {
    assert_eq!(commands::parse_command("q"), Some(Command::Quit));
}

#[test]
fn quit_command() {
    assert_eq!(commands::parse_command("quit"), Some(Command::Quit));
}

#[test]
fn quit_command_rejects() {
    assert_eq!(commands::parse_command("qs"), None);
    assert_eq!(commands::parse_command("quits"), None);
    assert_eq!(commands::parse_command("quit 1"), None);
}
