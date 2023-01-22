pub struct Response {
    pub message: String,
    pub exit_code: i32,
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub command: String,
    pub subcommand: String,
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

