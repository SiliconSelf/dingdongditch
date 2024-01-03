//! Code for parsing commands input by the user and returning them as
//! machine-readable enum variants.
//!
//! This works by comparing the input commands against a lazily-evaluated Vec of
//! Regex matchers. If any of the expressions are a match, only then will the
//! much more computationally expensive task of capturing arguments be
//! performed.
//!
//! This is almost certainly not the best way to do this, but it's simple and
//! serves our purposes for now. If needed, the most likely better solution is
//! bringing in an extra library like clap. Anything more complicated than this
//! approach will be done better by a dedicated tool.

use once_cell::sync::Lazy;
use regex::Regex;

/// The Vec of compiled regular expressions
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

/// The valid commands that can be entered by a user
#[derive(PartialEq)]
pub(crate) enum Command {
    /// Quit the program.
    Quit,
    /// Change networking interface to the provided name
    ChangeInterface(String),
    /// Toggle the listener on the selected interface
    Listen,
}

/// Possible errors that can be encountered while parsing a command
pub(crate) enum Errors {
    /// The command does not match any regex
    UnknownCommand,
}

impl TryFrom<String> for Command {
    type Error = Errors;

    /// Attempt to parse a user command provided as a String
    fn try_from(value: String) -> Result<Self, Self::Error> {
        for (pattern, command) in REGEXES.iter() {
            if pattern.is_match(&value) {
                // This variable will be used for commands with arguments later
                let captures = pattern.captures(&value).expect(
                    "This should always succeed because we already know the \
                     pattern matches",
                );
                return match command {
                    Command::Quit => Ok(Command::Quit),
                    Command::ChangeInterface(_) => {
                        Ok(Command::ChangeInterface(captures[1].to_owned()))
                    }
                    Command::Listen => Ok(Command::Listen),
                };
            }
        }
        Err(Errors::UnknownCommand)
    }
}
