use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else( |err| {
        println!("Problem while parsing config: {err}");
        process::exit(1); // process error exit.
    });
    let contents = fs::read_to_string(config.path.clone())
        .expect("Should have been able to read the file");
    println!("With text:\n\n{contents}");
}

struct Config {
    query: String,
    path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { // err handling.
            return Err("not enough args: expecting 3.");
        }
        let q = args[1].clone();
        let p = args[2].clone();
        Ok(Config { query: q, path: p })
    }
}
