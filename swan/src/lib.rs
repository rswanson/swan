use std::error::Error;
use swan_about;

pub struct Response {
    pub message: String,
    pub exit_code: i32,
}
pub struct Config {
    command: String,
    subcommand: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let command = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let subcommand = match args.next() {
            Some(arg) => arg,
            None => "".to_string(),
        };

        Ok(Config {
            command,
            subcommand,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    if config.command.eq("about") {
        swan_about::run();
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response() {
        let response = Response {
            message: "test".to_string(),
            exit_code: 0,
        };
        assert_eq!(response.exit_code, 0);
    }
}
