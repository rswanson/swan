use swan_common::{Config, Response};

pub fn run(config: Config) -> Response {
    let help_message = "usage: swan <command> <subcommand>".to_string();
    
    Response { message: help_message, exit_code: 0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
