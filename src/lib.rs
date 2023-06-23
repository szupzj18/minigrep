use std::{fs, error::Error};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)
    .expect("Should have been able to read the file");
    
    println!("result:");
    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str>{
    let mut res = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);        
        }
    }
    res
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

mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}