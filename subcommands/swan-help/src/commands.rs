use std::{collections::HashMap};

pub struct Command {
    pub command: &'static str,
    pub help_message: &'static str,
}

pub const help: Command = Command {
    command: "help",
    help_message: "usage: swan <command> <subcommand>",
};

pub const about: Command = Command {
    command: "about",
    help_message: "usage: swan about",
};

pub const creds: Command = Command {
    command: "creds",
    help_message: "usage: swan creds <cloud_provider>",
};

pub fn get_command_list<'a>() -> Box<HashMap<&'static str, Command>> {
    let mut commands = HashMap::new();

    commands.insert(help.command, help);
    commands.insert(about.command, about);
    commands.insert(creds.command, creds);

    Box::new(commands)
}
