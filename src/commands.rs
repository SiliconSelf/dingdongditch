use once_cell::sync::Lazy;
use regex::Regex;

static REGEXES: Lazy<Vec<(Regex, Command)>> = Lazy::new(|| {
    vec![
        (Regex::new("^q$").unwrap(), Command::Quit),
        (Regex::new("^quit$").unwrap(), Command::Quit),
        (Regex::new(r#"^i (\w+)$"#).unwrap(), Command::ChangeInterface(String::new())),
        (Regex::new(r#"^interface (\w+)$"#).unwrap(), Command::ChangeInterface(String::new()))
    ]
});

pub(crate) enum Command {
    Quit,
    ChangeInterface(String),
}

pub(crate) fn parse_command(input: String) -> Option<Command> {
    for (pattern, command) in &*REGEXES {
        if pattern.is_match(&input) {
            let caps = pattern.captures(&input).unwrap();
            return match command {
                Command::Quit => { Some(Command::Quit) },
                Command::ChangeInterface(_) => { Some(Command::ChangeInterface(caps[1].to_string()))
                },
            }
        }
    }
    None
}
