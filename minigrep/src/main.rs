use std::env;
use std::process;

use minigrep::Config;


fn main() {
    let args = env::args();  // The env::args function returns an iterator whose items are strings.

    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(-1);
    });

    // println!("Searching for '{}' in file '{}'.", config.query, config.file_path);  // debug output

    if let Err(err) = minigrep::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(-2);
    }
}
