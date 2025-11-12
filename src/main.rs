use std::env::args;
use std::fs;

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn new(args: &[String]) -> Self {
        Config{query: args[1].clone(), file_path: args[2].clone()}
    }
}
fn main() {
    println!("...:::mini-grep:::...");
    let args: Vec<String> = args().collect();
    let config = Config::new(&args);
    println!("Searching for query: {}", config.query);
    println!("In file: {}", config.file_path);

    let content = fs::read_to_string(config.file_path)
            .expect("should have been able to read this file");

    println!("{content}");
}
