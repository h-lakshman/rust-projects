use minigrep;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        println!("Error parsing arguements{err}");
        process::exit(1)
    });
    println!(
        "Searching for {} \nin file {}",
        config.search_query, config.file_path
    );
    if let Err(e) = minigrep::run(&config) {
        println!("Application error {e}");
        process::exit(1);
    }
}
