use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Config.
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsin arguments: {}", err);
        process::exit(1);
    });

    // Run.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
