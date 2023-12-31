//! Lmao

use once_cell::sync::Lazy;
use regex::Regex;

/// This Vec of compiled regular expressions is for parsing user commands. It's
/// not the ideal solution, but it does work for now.
static REGEXES: Lazy<Vec<(Regex, Command)>> = Lazy::new(|| {
    vec![
        (Regex::new(r"^q$").expect(""), Command::Quit),
        (Regex::new(r"^quit$").expect(""), Command::Quit),
        (
            Regex::new(r"^i (\w+)$").expect(""),
            Command::ChangeInterface(String::new()),
        ),
        (
            Regex::new(r"^interface (\w+)$").expect(""),
            Command::ChangeInterface(String::new()),
        ),
        (Regex::new(r"^l$").expect(""), Command::Listen),
        (Regex::new(r"^listen$").expect(""), Command::Listen),
    ]
});

#[derive(PartialEq, Debug)]
/// This enum represents every possible command the user can input along with
/// any arguments
pub(crate) enum Command {
    /// Quit the program
    Quit,
    /// Change what interface the program is using
    ChangeInterface(String),
    /// Toggle passive listening for host detection
    Listen,
}

/// Parse a provided String to try to find a command
pub(crate) fn parse_command(input: &str) -> Option<Command> {
    for (pattern, command) in &*REGEXES {
        if pattern.is_match(input) {
            let caps =
                pattern.captures(input).expect("We already know this matches");
            return match command {
                Command::Quit => Some(Command::Quit),
                Command::ChangeInterface(_) => {
                    Some(Command::ChangeInterface(caps[1].to_string()))
                }
                Command::Listen => Some(Command::Listen),
            };
        }
    }
    None
}
