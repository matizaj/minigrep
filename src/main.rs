use std::env::{args, var};
use std::process;
use std::error::Error;
use std::fs;
use mini_grep::{search, search_case_insensitive};

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let ignore_case = var("IGNORE_CASE").is_ok();
        return Ok(Config{query: args[1].clone(), file_path: args[2].clone(), ignore_case});
    }
}
fn main() {
    println!("...:::mini-grep:::...");
    let args: Vec<String> = args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = run(config) {
         println!("problem parsing arguments: {err}");
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
