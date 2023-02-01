use commands::{get_command_list, Command};
use swan_common::{Config, Response};

mod commands;

pub fn run(config: Config) -> Response {
    let commands = get_command_list();
    let mut command: &Command = &Command { command: "help", help_message: "usage: swan help <subcommand>\n displays help information about the specified subcommand", };
    if commands.get(config.subcommand.as_str()).is_some() {
        command = commands.get(config.subcommand.as_str()).unwrap();
    }
    Response {
        message: command.help_message.to_string(),
        exit_code: 0,
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
