use commands::{get_command_list, Command};
use swan_common::{Config, Response};

mod commands;

pub fn run(config: Config) -> Response {
    let command: &str = config.command.as_ref();
    let message = "usage";

    Response { message: message.to_string(), exit_code: 0 }
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
