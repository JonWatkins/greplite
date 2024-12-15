use tinygrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|error| {
        error.handle_error();
        process::exit(1);
    });

    if let Err(error) = tinygrep::run(config) {
        error.handle_error();
        process::exit(1);
    }
}
