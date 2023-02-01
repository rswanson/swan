#[derive(Debug, Clone, Copy)]
pub struct Command {
    pub command: &'static str,
    pub help_message: &'static str,
}

pub const DEFAULT: Command = Command {
    command: "default",
    help_message: "usage: swan <command> <subcommand>",
};

pub const HELP: Command = Command {
    command: "help",
    help_message: "usage: swan help <command>",
};

pub const ABOUT: Command = Command {
    command: "about",
    help_message: "usage: swan about",
};

pub const CREDS: Command = Command {
    command: "creds",
    help_message: "usage: swan creds <cloud_provider>",
};

impl Command {
    pub fn parse_command(command: String) -> Command {
        match command {
            _ if command == "about" => ABOUT,
            _ if command == "creds" => CREDS,
            _ if command == "help" => HELP,
            _ => DEFAULT,
        }
    }
}
