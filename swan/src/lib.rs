use std::error::Error;

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
        let _resp = about::run();
    }

    dbg!(config.subcommand);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response() {
        let response = common::Response {
            message: "test".to_string(),
            exit_code: 0,
        };
        assert_eq!(response.exit_code, 0);
    }
}
