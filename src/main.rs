use std::env::{args, var};
use std::process;
use std::error::Error;
use std::fs;
use matizaj_mini_grep::{search, search_case_insensitive};

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(q) => q,
            None => return Err("dont receive query string!")
        };

        let file_path = match args.next() {
            Some(path) => path,
            None => return Err("path to file not specified!")
        };

        let ignore_case = var("IGNORE_CASE").is_ok();
        Ok(Config{query, file_path, ignore_case})
    }
}
fn main() {
    println!("...:::mini-grep:::...");

    let config = Config::build(args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    }
    
}

fn run(config: Config) ->Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    
    for line in result {
        println!("{line}")
    }
    Ok(())
}
