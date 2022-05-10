use std::fs;
use std::error::Error;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("contents: {}", contents);

    Ok(());
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut  args: env::Args]) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }
        // let query = args[1].clone();
        // let filename = args[2].clone();

        let query = match args.next() {
            Some(arg) => arg,
            None => Err("did't get a query"),
        }

        let filename = match args.next() {
            Some(arg) => arg,
            None => Err("did't get a filename "),
        }

        // let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config { query, filename })
    }
}