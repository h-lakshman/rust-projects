use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let search_query = &args[1];
    let file_name = &args[2];
    println!("Searching for {} \nin file {}", search_query, file_name);
}
