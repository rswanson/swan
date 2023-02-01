use std::error::Error;

use swan_common::{Config, Response};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let resp = match config.command {
        _ if config.command == "about" => swan_about::run(),
        _ if config.command == "creds" => Response {
            message: "creds".to_string(),
            exit_code: 0,
        },
        _ if config.command == "help" => swan_help::run(config),
        _ => swan_help::run(config),
    };
    println!("{}", resp.message);
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
