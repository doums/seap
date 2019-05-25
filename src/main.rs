use std::env;
use std::process;
use seap::parser::Config;

fn main() {
    let mut config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("parser error: {}", err);
        process::exit(1);
    });
}
