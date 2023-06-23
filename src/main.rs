use std::{env, process};
use minigrep::{Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else( |err| {
        eprintln!("Problem while parsing config: {err}"); // use {eprintlin!} error goes to io::stderr
        process::exit(1); // process error exit.
    });
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error! {e}");
        process::exit(1);
    }
}
