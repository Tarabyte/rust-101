extern crate minigrep;

use std::env;
use std::process;
use minigrep::Config;


fn main() {
    let config = Config::from_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing args `{}`", err);

        process::exit(1)
    });

    println!("Seaching for {} in {}.", &config.query, &config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Error running application: {}", e);

        process::exit(1);
    };
}
