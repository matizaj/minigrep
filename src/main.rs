use std::env::args;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        return Ok(Config{query: args[1].clone(), file_path: args[2].clone()});
    }
}
fn main() {
    println!("...:::mini-grep:::...");
    let args: Vec<String> = args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for query: {}", config.query);
    println!("In file: {}", config.file_path);

    let content = fs::read_to_string(config.file_path)
            .expect("should have been able to read this file");

    println!("{content}");
}
