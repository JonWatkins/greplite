use std::{env, process};
use greplite::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|error| {
        error.handle_error();
        process::exit(1);
    });

    if let Err(error) = greplite::run(config) {
        error.handle_error();
        process::exit(1);
    }
}
