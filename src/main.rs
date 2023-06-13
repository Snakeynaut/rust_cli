use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let current_direc = env::current_dir()
        .expect("Unable to read current directory")
        .to_str().expect("Unable to open directory").to_string();

    println!("Searching for {}", config.query);
    println!("In file {}", config.filepath);
    println!("Current directory is {:?}", current_direc);

    let contents = fs::read_to_string(config.filepath)
        .expect("Should have been able to read file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    filepath: String,
}


impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filepath = args[2].clone();

        Config {query, filepath};
    }
}