use std::{fs, error::Error};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)
    .expect("Should have been able to read the file");
    println!("With text:\n\n{contents}");
    Ok(())
}

#[allow(unused)]
pub struct Config {
    query: String,
    path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { // err handling.
            return Err("not enough args: expecting 3.");
        }
        let q = args[1].clone();
        let p = args[2].clone();
        println!("searching for: {q}");
        println!("file path : {p}");
        Ok(Config { query: q, path: p })
    }
}