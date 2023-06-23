use std::{fs, error::Error, env};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)
    .expect("Should have been able to read the file");
    
    println!("result:");
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
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

pub fn search_case_insensitive<'a>(
    query: &'a str, 
    contents: &'a str
) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[allow(unused)]
pub struct Config {
    query: String,
    path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { // err handling.
            return Err("not enough args: expecting 3.");
        }
        let q = args[1].clone();
        let p = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        println!("searching for: {q}");
        println!("file path : {p}");
        println!("ignore case : {ignore_case}");
        Ok(Config { 
            query: q, 
            path: p, 
            ignore_case: ignore_case
        })
    }
}

#[cfg(test)]
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

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."], 
            search_case_insensitive(query, contents)
        );
    }
}