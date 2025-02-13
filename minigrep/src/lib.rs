use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub search_query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let search_query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            search_query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.search_query, &file_contents)
    } else {
        search(&config.search_query, &file_contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(search_query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(search_query) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(search_query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = search_query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line.trim());
        }
    }

    result
}
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust
Safe,fast,productive.
ick three.
Duct Tape.";

        assert_eq!(vec!["Safe,fast,productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let contents = "\
Rust
Safe,fast,productive.
ick three.
Trust me.";

        assert_eq!(
            vec!["Rust", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
