use core::{Config, Response};
use std::env;
use std::process;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let sample_response = Response {
        message: "Hello, World!".to_string(),
        exit_code: 0,
    };
    println!("{}", sample_response.message);
}
