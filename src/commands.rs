pub(crate) enum Command {
    ChangeInterface(String),
    Quit,
}

pub(crate) fn parse_command(input: String) -> Option<Command> {
    Some(Command::Quit)
}
