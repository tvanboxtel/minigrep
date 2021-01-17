use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let vars: Vec<String> = env::args().skip(1).collect();
    let config = Config::new(&vars).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1)
    }
}
