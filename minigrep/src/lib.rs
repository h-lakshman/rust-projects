use std::error::Error;
use std::fs;

pub struct Config {
    pub search_query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let search_query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config {
            search_query,
            file_path,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_contents =
        fs::read_to_string(&config.file_path).expect("Should have been able to read the file");

    for line in search(&config.search_query, &file_contents) {
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
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust
Safe,fast,productive.
ick three.";

        assert_eq!(vec!["Safe,fast,productive."], search(query, contents))
    }
}
