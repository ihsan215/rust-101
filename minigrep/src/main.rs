use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    query:String,
    file_path:String,
}


impl Config {
    fn new(args:&[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn parse_config(args:&[String]) -> Config{
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config { query, file_path }
}

 fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}


fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}