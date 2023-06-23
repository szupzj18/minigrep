use std::{env, process};
use minigrep::{Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else( |err| {
        println!("Problem while parsing config: {err}");
        process::exit(1); // process error exit.
    });
    
    if let Err(e) = minigrep::run(config) {
        println!("Application Error! {e}");
        process::exit(1);
    }
}
