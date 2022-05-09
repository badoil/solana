use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::parse_config(&args);

    println!("query: {}", config.query);
    println!("filename: {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("something went wrong");

    println!("contents: {}", contents);

}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn parse_config(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Config { query, filename }
    }
}