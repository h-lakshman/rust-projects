use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguements{err}");
        process::exit(1)
    });
    println!(
        "Searching for {} \nin file {}",
        config.search_query, config.file_path
    );
    if let Err(e) = run(&config) {
        println!("Application error {e}");
        process::exit(1);
    }
}

struct Config {
    search_query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
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

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let file_contents =
        fs::read_to_string(&config.file_path).expect("Should have been able to read the file");
    println!("Contents: \n{}", file_contents);
    Ok(())
}
