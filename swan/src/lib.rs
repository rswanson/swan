use std::error::Error;
#[derive(Debug, PartialEq)]
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
    fn test_build_config() {
        let valid_config = Config {
            command: "about".to_string(),
            subcommand: "test".to_string(),
        };
        let args = vec!["swan".to_string(), "about".to_string(), "test".to_string()];
        let config = Config::build(args.into_iter());
        let config = config.unwrap();
        assert_eq!(valid_config, config);
    }

    #[test]
    fn test_run() {
        let valid_config = Config {
            command: "about".to_string(),
            subcommand: "test".to_string(),
        };
        assert!(run(valid_config).is_ok());
    }
}
