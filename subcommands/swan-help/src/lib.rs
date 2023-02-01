use commands::Command;
use swan_common::{Config, Response};

mod commands;

pub fn run(config: Config) -> Response {
    let command = Command::parse_command(config.command);
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
