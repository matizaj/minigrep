use std::env::args;
use std::process;
use std::error::Error;
use std::fs;
use mini_grep::search;

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

    if let Err(err) = run(config) {
         println!("problem parsing arguments: {err}");
        process::exit(1);
    }
    
}

fn run(config: Config) ->Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let result = search(&config.query, &content);
    println!("{:?}", result);
    Ok(())
}
