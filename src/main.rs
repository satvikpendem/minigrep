use std::{env, process};

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {} in {}", config.query, config.filename);

    if let Err(e) = config.run() {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
